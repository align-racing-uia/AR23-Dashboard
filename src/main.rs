pub mod helper;
pub mod ar23;
pub mod align_dbc;
pub mod cascadia_dbc;

use std::{fs::File, io::Read};
use ar23::*;
use socketcan::*;
use std::thread;

fn main(){
    
    let width = 800;
    let height = 480;

    // Load instance of endurance screen
    let endu = EnduranceScreen::new(width, height);

    // Load instance of debug screen
    let dbg = DebugScreen::new(width, height); 

    //let mut buffer: Vec<u32> = vec![];
    let mut gui = AR23GUI::new(width, height, vec![
        Box::new(endu),
        Box::new(dbg),
    ]);

    

    gui.run();
    

}

