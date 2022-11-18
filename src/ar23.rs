use crate::{helper::*, cascadia_dbc::MessagesCascadia};
use minifb::*;
use std::thread;
use socketcan::*;
use crossbeam_channel::{bounded};

use crate::align_dbc::*;

pub struct AR23GUI {
    window: Window, 
    width: u32,
    height: u32,
    screens: Vec<Box<dyn AR23Screen>>,
    screen_index: usize
}

impl AR23GUI {
    pub fn new(width: u32, height: u32, screens: Vec<Box<dyn AR23Screen>>) -> Self {
        let options = WindowOptions {
            borderless: true,
            
            ..Default::default()
        };
        let mut window = Window::new("AR23 GUI", width as usize, height as usize, options).unwrap();
        AR23GUI { window: window, width: width, height: height, screens: screens, screen_index: 0 }
    }


    pub fn draw_screen(&mut self) -> Drawn {
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

    pub fn should_close(&self) -> bool {
        // returns true if windows is closed, OR escape key is pressed
        return !self.window.is_open() || self.window.is_key_down(Key::Escape);
    }

    fn handle_data(&mut self, data_vector: Vec<CANFrame>){
        self.screens[self.screen_index].update(data_vector);
        //println!("Frame updated!");
    }

    pub fn run(&mut self) {

        // creating channel to pass information about canbus back and forth
        let (send, recieve) = bounded(0);

        // spawn thread for reading the canbus.
        thread::spawn( move ||{
            let socket = CANSocket::open("vcan0").expect("vcan0, did not exist");
            let mut last_message: CANFrame;
            let mut frame_vector: Vec<CANFrame> = Vec::new();
            loop {
                // collect all canbus messages with minimum delay
                last_message = socket.read_frame().unwrap();

                // hopefully this should become too big.. (max around 100 messages at once)
                frame_vector.push(last_message);
                match send.try_send(frame_vector.clone()) {
                    Ok(()) => {
                        // if can frame buffer is sent, empty it
                        println!("Emptied frame_vector when it had: {}", frame_vector.len());
                        frame_vector = Vec::new();
                    },
                    _ => {}
                }
            }
        });

        while !self.should_close() {
            let now = std::time::Instant::now();
            self.draw_screen();
            println!("{} ms to draw current screen",now.elapsed().as_millis());
            //blocking recv -- no point in updating screen, if there is no new data
            let data = recieve.recv().unwrap();
            //blocking data handling - give necisarry data to the current screen
            let now = std::time::Instant::now();
            self.handle_data(data);
            println!("{} ms to process canframes",now.elapsed().as_millis());
        }
    }

}

pub trait AR23Screen {
    
    fn draw(&mut self) -> &Buffer;
    fn update(&mut self, data: Vec<CANFrame>);
    
}

// ENDURANCE SCREEN PROGRAMMING
pub struct EnduranceScreen {
    buffer: Buffer,
    status: String,
    current_speed: i16,
}

impl EnduranceScreen {
    pub fn new(w: u32, h: u32) -> Self {
        EnduranceScreen { 
            buffer: Buffer::new(w, h),
            status: String::from("Everything ok"),
            current_speed: 0
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

    fn draw_content(&mut self) {
        let width = self.buffer.width as i32;
        let height = self.buffer.height as i32;
        let speed_text = Text::new(&self.buffer, Point { x: width/2 , y: height/2 }, format!("{} km/t", self.current_speed).as_str(), 40.0, Color::BLACK);
        speed_text.draw(&mut self.buffer);
    }

}

impl AR23Screen for EnduranceScreen {

    fn update(&mut self, data: Vec<CANFrame>) {
        for data_point in data {

            // handling apps messages:
            match MessagesAlign::from_can_message(data_point.id(), data_point.data()) {
                Ok(apps) => {
                   match apps {
                        MessagesAlign::Apps(data) => {
                            println!("GOT DATA!!");
                            println!("Ready to drive: {:?}", data.ready_to_drive());
                        },
                        _ => {}
                   } 
                }
                _ => {}
            };

            // handeling cascadia messages:
            match MessagesCascadia::from_can_message(data_point.id(), data_point.data()) {
                Ok(cascadia) => {
                    match cascadia {
                        MessagesCascadia::M165MotorPositionInfo(data) => {
                            self.current_speed = (data.d2_motor_speed() as f32 * 0.0314256).round() as i16;
                        }
                        _ => {} // ignore the rest
                    }
                },
                _ => {}
            }


        }
    }

    fn draw(&mut self) -> &Buffer {

        self.buffer.clear(Color::LIGHT_GRAY);
        self.draw_header();
        self.draw_content();

        return &self.buffer;
    }
}

pub struct DebugScreen {
    buffer: Buffer
}

impl DebugScreen {
    pub fn new(w: u32, h: u32) -> Self {
        DebugScreen { buffer: Buffer::new(w, h) }
    } 



}

impl AR23Screen for DebugScreen {

    fn update(&mut self, data: Vec<CANFrame>) {
        
    }

    fn draw(&mut self) -> &Buffer {

        self.buffer.clear(Color::LIGHT_GRAY);

        return &self.buffer;
    }
}



#[derive(PartialEq)]
pub enum Drawn {
    Ok,
    Err
}

impl Color {
    pub const LIGHT_GRAY: u32 = 0xBBBBBB;
}

