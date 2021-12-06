use crate::jobs::{ config::load_config, Config };
// use super::util::{ EventError };

pub async fn init() /*-> Result<(), EventError>*/{
    let config: Config = load_config().await;


}
