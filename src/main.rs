use minifb::*;

pub mod helper;
use helper::*;

fn main(){
    
    //let mut buffer: Vec<u32> = vec![];
    let mut gui = AR23GUI::new(800, 480);

}


struct AR23GUI {
    window: Window, 
    width: u32,
    height: u32,
    screens: Vec<AR23Screen>,
    screen_index: usize
}

impl AR23GUI {
    fn new(width: u32, height: u32) -> Self {
        let options = WindowOptions {
            borderless: true, 
            ..Default::default()
        };
        let mut window = Window::new("AR23 GUI", width as usize, height as usize, options).unwrap();
        AR23GUI { window: window, width: width, height: height, screens: Vec::new(), screen_index: 0 }
    }

    fn append_screen(&mut self, screen: AR23Screen) {
        self.screens.push(screen);
    }

    fn draw_screen(&mut self) -> &Buffer {
        if 0 >= self.screens.len() {
            panic!("No screens added!");
            return &Buffer::new(self.width, self.height);
        }
        
        return &self.screens[self.screen_index].buffer;
    }
}

struct AR23Screen {
    buffer: Buffer
}

impl AR23Screen {
    fn new(b: Buffer) -> Self {
        AR23Screen {
            buffer: b
        }
    }
}



