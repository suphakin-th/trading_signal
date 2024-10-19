use base::error;
use base::util;
use snafu::prelude::*;

use std::{convert::TryInto, path::PathBuf};
use base::configuration::environment::Environment;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
	pub log_level: String,
}

pub async fn get_configuration(base_path: Option<PathBuf>) -> util::Result<Settings> {
	let environment: Environment = std::env::var("APP_ENVIRONMENT")
		.unwrap_or_else(|_| "local".into())
		.try_into()?;
	let environment_filename = format!("{}.toml", environment.as_str());
	let configuration_directory = base_path.unwrap_or(
		std::env::current_dir()
			.context(error::PathEnvSnafu)?
			.join("configuration")
			.join(environment_filename),
	);
	let settings = config::Config::builder()
		.add_source(config::File::from(configuration_directory))
		.add_source(
			config::Environment::with_prefix("APP")
				.prefix_separator("_")
				.separator("__"),
		)
		.build()
		.context(error::ConfigEnvSnafu)?;

	settings
		.try_deserialize::<Settings>()
		.context(error::ConfigEnvSnafu)
}