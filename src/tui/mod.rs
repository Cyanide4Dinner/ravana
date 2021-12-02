use libnotcurses_sys::{
    Nc,
    NcDim,
    NcResult
};

//DEV
pub fn test_tui(){
    let nc = unsafe{ Nc::new().unwrap() };    
    init(nc).unwrap();
    render(nc).unwrap();
    stop(nc);
}

fn init(nc: &mut Nc) -> NcResult<NcDim>{
   let plane = unsafe{ nc.stdplane() }; 
   plane.putstr("hello world")
}

fn render(nc: &mut Nc) -> NcResult<()>{
   nc.render() 
}

fn stop(nc: &mut Nc) {
   unsafe{ nc.stop().unwrap() };
}
