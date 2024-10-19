use std::io;

use config::ConfigError;
use reqwest::Error as ReqErr;
use snafu::{prelude::*, Report};
use time::error::ComponentRange;
use tracing::metadata::ParseLevelFilterError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
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