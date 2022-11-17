use minifb::*;

pub mod helper;
use helper::*;

fn main(){
    
    let endu = EnduranceScreen {
        buffer: Buffer::new(800, 480)
    };
    //let mut buffer: Vec<u32> = vec![];
    let mut gui = AR23GUI::new(800, 480, vec![
        Box::new(endu)

    ]);

}


struct AR23GUI {
    window: Window, 
    width: u32,
    height: u32,
    screens: Vec<Box<dyn AR23Screen>>,
    screen_index: usize
}

impl AR23GUI {
    fn new(width: u32, height: u32, screens: Vec<Box<dyn AR23Screen>>) -> Self {
        let options = WindowOptions {
            borderless: true, 
            ..Default::default()
        };
        let mut window = Window::new("AR23 GUI", width as usize, height as usize, options).unwrap();
        AR23GUI { window: window, width: width, height: height, screens: screens, screen_index: 0 }
    }


    fn draw_screen(&mut self) -> &Buffer {
        if 0 >= self.screens.len() {
            panic!("No screens added!");
            return &Buffer::new(self.width, self.height);
        }
        
        return &self.screens[self.screen_index].buffer();
    }
}

trait AR23Screen {
    
    fn update(&mut self);
    fn buffer(&self) -> &Buffer;
    
}



struct EnduranceScreen {
    buffer: Buffer
}

impl AR23Screen for EnduranceScreen {
    fn update(&mut self) {
        
    }

    fn buffer(&self) -> &Buffer {
        return &self.buffer;
    }
}



