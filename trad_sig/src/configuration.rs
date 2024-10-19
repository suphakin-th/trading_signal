use time::format_description::FormatItem;
use tokio::sync::OnceCell;

use self::setting::Settings;

pub mod setting;

pub const DATE_TIME_FORMAT: &[FormatItem<'_>] =
	time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

pub const DATE_TIME_WITH_SUBSECOND_FORMAT: &[FormatItem<'_>] =
	time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]");

pub static SETTINGS: OnceCell<Settings> = OnceCell::const_new();
