pub mod helper;
pub mod ar23;

use std::{fs::File, io::Read};
use ar23::*;
use can_dbc::{*, parser::dbc};

fn main(){
    
    let width = 800;
    let height = 480;

    let mut endu = EnduranceScreen::new(width, height);
    let mut dbg = DebugScreen::new(width, height); 

    //let mut buffer: Vec<u32> = vec![];
    let mut gui = AR23GUI::new(width, height, vec![
        Box::new(endu),
        Box::new(dbg),
    ]);

    let mut f = File::open("./elcon.dbc").expect("Could not open");
    let mut dbc_string = String::from("");
    f.read_to_string(&mut dbc_string).expect("Could not read file");
    let mut dbc = can_dbc::DBC::try_from(dbc_string.as_str()).expect("Could not parse. Are the order correct?");

    for message in dbc.messages() {
        println!("{:?}", message);
    }
    while gui.draw_screen() == Drawn::Ok && gui.should_close() {
        
    }

}

