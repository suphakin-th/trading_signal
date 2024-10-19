
use base::util;
use std::thread;
use std::time::Duration;
use crate::service::zmq;

pub fn run() -> util::Result<()> {
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

    // //Receive message
	// let zmq = zmq::Zmq::new();
	// let subscriber = zmq.create_subscriber("tcp://127.0.0.1:5555", "");

	// println!("Receiveeeeeee started...");
	// loop {
	// 	let message = subscriber.receive_message();
    //     println!("Received: {}", message);
	// }
}
