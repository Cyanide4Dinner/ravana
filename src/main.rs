use env_logger;

//mod api;
mod def;
mod tui;
mod events;
mod jobs;
mod input;
mod tools;

fn main(){
    env_logger::init();
    events::lifecycle::ravana().expect("Main loop error.");
}   
