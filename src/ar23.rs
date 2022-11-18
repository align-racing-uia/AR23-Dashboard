
pub struct AR23GUI {
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


    fn draw_screen(&mut self) -> Drawn {
        if 0 >= self.screens.len() {
            panic!("No screens added!");
            return Drawn::Err;
        }
        let mut ok = true;
        
        let draw_buffer = self.screens[self.screen_index].draw();
        self.window.update_with_buffer(&draw_buffer.to_buffer(), self.width as usize, self.height as usize).unwrap_or_else(|e|{
            ok = false;
        });
        
        if ok {
            return Drawn::Ok;
        }
        return Drawn::Err;
    }

    fn should_close(&self) -> bool {
        return self.window.is_open() && !self.window.is_key_down(Key::Escape);
    }
}

pub trait AR23Screen {
    
    fn draw(&mut self) -> &Buffer;
    fn update(&mut self);
    
}

// ENDURANCE SCREEN PROGRAMMING
pub struct EnduranceScreen {
    buffer: Buffer,
    status: String,
}

impl  EnduranceScreen {
    fn new(w: u32, h: u32) -> Self {
        EnduranceScreen { 
            buffer: Buffer::new(w, h),
            status: String::from("Everything ok")
        }
    } 

    fn draw_header(&mut self) {
        let width = self.buffer.width as i32;
        Draw::rect_fill(&mut self.buffer, &Point { x: 0, y: 0 }, &Point { x: width / 2, y: 80 }, Color::GREEN);
        Draw::rect_fill(&mut self.buffer, &Point { x: width / 2 + 10, y: 0 }, &Point { x: width / 6 * 4, y: 80 }, Color::GREEN);
        Draw::rect_fill(&mut self.buffer, &Point { x: width / 6 * 4 + 10, y: 0 }, &Point { x: width / 6 * 5, y: 80 }, Color::GREEN);
        Draw::rect_fill(&mut self.buffer, &Point { x: width / 6 * 5 + 10, y: 0 }, &Point { x: width, y: 80 }, Color::GREEN);

        let mut status_text = Text::new(&self.buffer, Point { x: width / 20, y: 20 }, &self.status, 40.0, Color::BLACK);
        status_text.draw(&mut self.buffer);
    }

}

impl AR23Screen for EnduranceScreen {

    fn update(&mut self) {
        
    }

    fn draw(&mut self) -> &Buffer {

        self.buffer.clear(Color::LIGHT_GRAY);
        self.draw_header();

        return &self.buffer;
    }
}

struct DebugScreen {
    buffer: Buffer
}

impl DebugScreen {
    fn new(w: u32, h: u32) -> Self {
        DebugScreen { buffer: Buffer::new(w, h) }
    } 



}

impl AR23Screen for DebugScreen {

    fn update(&mut self) {
        
    }

    fn draw(&mut self) -> &Buffer {

        self.buffer.clear(Color::LIGHT_GRAY);



        return &self.buffer;
    }
}



#[derive(PartialEq)]
enum Drawn {
    Ok,
    Err
}

impl Color {
    pub const LIGHT_GRAY: u32 = 0xBBBBBB;
}

