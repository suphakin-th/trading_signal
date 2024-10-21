use base::{error, util};
use snafu::OptionExt;
use std::{future::Future, thread};
use std::time::Duration;
use tokio::{sync::broadcast, task::LocalSet};
use crate::service::zmq;
use tokio::time::{self, Instant, Interval, MissedTickBehavior};

use std::time::SystemTime;

fn saturating_micros_duration(micros: u128) -> Duration {
	Duration::from_micros(micros.try_into().unwrap_or(u64::MAX))
}

pub fn time_aligned_duration(period: Duration) -> Duration {
	let micros = saturating_micros_duration(period.as_micros());

	match SystemTime::UNIX_EPOCH.elapsed() {
		Ok(s) => micros - saturating_micros_duration(s.as_micros() % micros.as_micros()),
		Err(e) => saturating_micros_duration(e.duration().as_micros() % micros.as_micros()),
	}
}

async fn shutdown_handler(
	snd    : broadcast::Sender<()>,
	mut rcv: broadcast::Receiver<()>,
) -> Result<(), error::SignalError<()>> {
	use tokio::signal::unix::{self, SignalKind};

	let mut send    = |sig| snd.send(sig).map(|_| ()).map_err(SignalError::Broadcast);
	let mut sigint  = unix::signal(SignalKind::interrupt()).map_err(SignalError::Handler)?;
	let mut sigterm = unix::signal(SignalKind::terminate()).map_err(SignalError::Handler)?;

	tokio::select! {
		Some(_) = sigint.recv()  => (),
		Some(_) = sigterm.recv() => (),
		sig     = rcv.recv()     => if sig.is_ok() { return Ok(()) },
	}

	send(())
}

pub fn shutdown_service(
	cap: usize,
) -> (
	broadcast::Sender<()>,
	broadcast::Receiver<()>,
	impl Future<Output = Result<(), SignalError<()>>>,
) {
	let (rad_tx, rad_rx) = broadcast::channel(cap);
	let svc_rad_tx = rad_tx.clone();
	let svc_rad_rx = rad_tx.subscribe();

	(rad_tx, rad_rx, shutdown_handler(svc_rad_tx, svc_rad_rx))
}

pub fn time_aligned_interval(period: Duration) -> Option<Interval> {
	let aligned = time_aligned_duration(period);
	let instant = Instant::now().checked_add(aligned)?;
	let mut interval = time::interval_at(instant, period);
	interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

	Some(interval)
}

pub async fn run() -> util::Result<()> {
	let mut task_interval =
		time_aligned_interval(Duration::from_secs(36000)).context(error::PeriodSnafu)?;
	let (interval_tx, _interval_rx) = broadcast::channel::<()>(100);
	//create task that run every 1 hour for get data and publish
	task_interval.tick().await;
	let local_worker = LocalSet::new();

	let (sig_tx, mut sig_rx, sig_svc) = sig::shutdown_service(32);
	let local_worker = LocalSet::new();

	// get information for CDC Action Zone trigger point
	// 	- If it same with previous value just ignore it.
	//	- Previous value use RWLock for keep value between task.
	local_worker
		.run_until(async {
			loop {
				tokio::select! {
					Ok(_) = sig_rx.recv() => {
						tracing::info!("Shutdown signal received.");
						break;
					},
					Ok(Err(e)) = &mut sig_svc, if !sig_svc.is_finished() => {
						tracing::error!("{}", e);
						sig_svc_err = true;
						if sig_tx.send(()).is_err() {
							tracing::error!("Failed to broadcast emergency shutdown signal.");
							force_abort = true;
						}
						break;
					},
					_ = task_interval.tick() => {
						// Check old task is finished if not just pass and wait for new tick
						if let Err(e) = interval_tx.send(()).context(error::SendUnitSnafu) {
							e.report();
						}
					}
				}
			}
		})
		.await;

	if !force_abort {
		tracing::info!("Services ended.");
		local_worker.await;
		sig_svc.abort();
	} else {
		tracing::info!("Shutdown forced.");
	}
	//Send message
	let zmq = zmq::Zmq::new();
	let publisher = zmq.create_publisher("tcp://127.0.0.1:5555");

	println!("Publisher started...");

	let mut count = 0;
	loop {
		let s_m = format!("Hello Subscriber, message #{}", count);

		publisher.send_message(&s_m);
		println!("Sent: {}", s_m);
		count += 1;
		thread::sleep(Duration::from_secs(2));
	}

	Ok(())
	// //Receive message
	// let zmq = zmq::Zmq::new();
	// let subscriber = zmq.create_subscriber("tcp://127.0.0.1:5555", "");

	// println!("Receiveeeeeee started...");
	// loop {
	// 	let message = subscriber.receive_message();
	//     println!("Received: {}", message);
	// }
}
