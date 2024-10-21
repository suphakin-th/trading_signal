use std::str::FromStr;

use base::{error, model::config_model, util};
use clap::Parser;

use snafu::{OptionExt, ResultExt};
use trad_sig::configuration::{setting, SETTINGS};

use tracing::metadata::LevelFilter;

#[tokio::main]
async fn main() -> util::Result<()> {
	let arg = config_model::CliCommand::parse();
	let config = setting::get_configuration(arg.config_file).await?;
	let setting = SETTINGS.get_or_init(|| async move { config }).await;
	let sub = tracing_subscriber::fmt()
		.compact()
		.with_file(true)
		.with_line_number(true)
		.with_thread_ids(true)
		.with_target(false)
		.with_max_level(
			LevelFilter::from_str(&setting.log_level).context(error::LevelFilterSnafu)?,
		)
		.finish();
	tracing::subscriber::set_global_default(sub)
		.ok()
		.context(error::GlobalDefautSnafu)?;
	trad_sig::app::run()
}
