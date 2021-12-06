use anyhow::{ Result };
use crate::jobs::{ config::load_config, Config };

pub async fn init() -> Result<()> {
    let config: Config = load_config().await;
    Ok(())
}
