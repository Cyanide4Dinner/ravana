use env_logger;

//use tokio::task;
//mod api;
pub mod def;
mod tui;
mod events;
pub mod jobs;
pub mod input;


#[tokio::main]
async fn main(){
    env_logger::init();
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // let future = api::call_api();
    // rt.block_on(async { future });

    //api::call_api();
    //api::oauth::oauth_process();
    // tui::test_tui();
    events::lifecycle::init().await.unwrap();
    // events::lifecycle::test_tui().unwrap();
}   

