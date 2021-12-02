use anyhow::{ Result, Context };

use crate::jobs::config::load_config;
use super::util::{ EventError };

pub async fn init() -> Result<(), EventError>{
    load_config().await;
    Ok(())
}
