use minifb::*;

pub mod helper;
pub mod ar23;

use helper::*;

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

    while gui.draw_screen() == Drawn::Ok && gui.should_close() {
        
    }

}

