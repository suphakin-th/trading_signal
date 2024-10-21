use std::{fmt, io};
use tokio::sync::broadcast;
use config::ConfigError;
use reqwest::Error as ReqErr;
use snafu::{prelude::*, Report};
use time::error::ComponentRange;
use tracing::metadata::ParseLevelFilterError;
use thiserror::Error;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
	#[snafu(display("Unable to create interval period"))]
	PeriodError,
	#[snafu(display("config error"))]
	ConfigEnv { source: ConfigError },
	#[snafu(display("path error"))]
	PathEnv { source: io::Error },
	#[snafu(display("set oncecell tokio pg error"))]
	LevelFilterError { source: ParseLevelFilterError },
	#[snafu(display("unsupport env"))]
	UnsupportEnv,
	#[snafu(display("tracing global default error"))]
	GlobalDefautError,
	#[snafu(display("overflow error"))]
	Overflow,
	#[snafu(display("serde json error"))]
	SerdeJsonError { source: serde_json::Error },
	#[snafu(display("invalid timestamp"))]
	NaiveDateTimeError,

	#[snafu(display("reqwest error"))]
	ReqwestError { source: ReqErr },
	#[snafu(display("reqwest clone error"))]
	ReqwestCloneError,
	#[snafu(display("reqwest header value error"))]
	ReqwestHeaderValueError {
		source: reqwest::header::InvalidHeaderValue,
	},
	#[snafu(display("component range error"))]
	ComponentRangeError { source: ComponentRange },
}

impl Error {
	pub fn report(&self) {
		tracing::error!("error: error_msg {}", Report::from_error(self))
	}
}

#[derive(Debug, Error)]
pub enum SignalError<T>
where
	T: 'static + fmt::Debug,
{
	#[error("Failed to register signal handler.")]
	Handler(#[source] io::Error),
	#[error("Failed to broadcast signal.")]
	Broadcast(#[source] broadcast::error::SendError<T>),
}
