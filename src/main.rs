use env_logger;
use log::debug;

//use tokio::task;
//mod api;
pub mod def;
mod tui;
mod events;
pub mod jobs;
pub mod input;
pub mod state;
pub mod tools;

fn main(){
    env_logger::init();
    debug!("Starting.");
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // let future = api::call_api();
    // rt.block_on(async { future });

    //api::call_api();
    //api::oauth::oauth_process();
    // tui::test_tui();
    events::lifecycle::init().await.unwrap();
    // events::lifecycle::test_tui().unwrap();
}   

