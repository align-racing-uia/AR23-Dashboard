use std::ops;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use fontdue::*;
use fontdue::layout::Layout;
use fontdue::layout::TextStyle;

pub struct Buffer {
    buffer: Vec<Vec<u32>>,
    pub width: u32,
    pub height: u32,
    pub clear_color: u32,
    font: Font,
}

impl Buffer {
    
    pub fn new(width: u32, height: u32) -> Self {

        println!("{:?}", env::current_dir().unwrap());

        let mut font_file = include_bytes!("./victor-pixel.ttf") as &[u8];

        let font = Font::from_bytes(font_file, fontdue::FontSettings::default()).unwrap();
  
        Buffer { 
            buffer: vec![vec![Color::WHITE;width as usize];height as usize], 
            width: width, 
            height: height,
            clear_color: Color::WHITE,
            font: font
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![vec![self.clear_color;self.width as usize]; self.height as usize];
    }

    // makes it possible to fetch color, without having to check if coord is whitin bounds every time
    pub fn get(&self, x: i32, y: i32) -> Found{

        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return Found::NOT;
        }

        return Found::OK(self.buffer[y as usize][x as usize]);

    }

    // helper function to skip having to check if coords is whitin bounds SO often. 
    // Also helps ignoring negative index numbers, which makes drawing lines that continoues outside bounds alot easier.
    pub fn set(&mut self, x: i32, y: i32, new: u32) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        self.buffer[y as usize][x as usize] = new;
    }

    pub fn size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }

    pub fn to_buffer(&self) -> Vec<u32> {
        return self.buffer.clone().into_iter().flatten().collect()
    }
}

// Implements indexing for the buffer struct
impl ops::Index<usize> for Buffer {
    type Output = Vec<u32>;
    fn index(&self, index: usize) -> &Self::Output {   
        return &self.buffer[index];
    }
}

impl ops::IndexMut<usize> for Buffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.buffer[index];  
    }
}


pub struct Text {
    text: String,
    pos: Point,
    layout: Layout
}

impl Text {

    pub fn new(buffer: &Buffer, pos: Point, input_text: &str) -> Self{
        let text = String::from(input_text);
        let font = buffer.font.clone();
        let mut text_layout = Layout::new(layout::CoordinateSystem::PositiveYDown);
        text_layout.append(&[font], &TextStyle::new(input_text, 18.0, 0));
        return Text {
            text: text,
            pos: pos,
            layout: text_layout
        };
    }

    pub fn draw(&mut self, buffer: &mut Buffer) {
        let mut x_offset = 0;
        println!("{:?}", self.layout.glyphs());
        for g in self.layout.glyphs() {

        }
    }

}

pub struct Draw;

impl Draw {
    
    // Who thought this one would be the hardest one :p

    pub fn line(buffer: &mut Buffer, p1: &Point, p2: &Point, color: u32, width: u32) {

        let mut np1 = p1;
        let mut np2 = p2;

        // make sure the first point is always the first on the x axis (for easier drawing conditions)
        if np1.x > np2.x {
            np1 = p2;
            np2 = p1;
        }
        
        // Calculus here we go
        let delta_x = np2.x as f32 - np1.x as f32;
        let delta_y = np2.y as f32 - np1.y as f32;
        

        // If line goes straight up, its a special case.
        if delta_x == 0.0 {
            // Either line would be invisible, or line would be completely out of bounds (saves a bit of processing)
            if delta_y == 0.0 || np1.x >= buffer.width as i32 {
                return;
            }

            let min_y = i32::min(np1.y, np2.y);
            let max_y = i32::max(np1.y, np2.y);

            for y in min_y..max_y+1 {
                let offset = (width / 2) as i32;
                for i in 0..width as i32{
                    buffer.set(np1.x-offset+i, y, color);
                }
                
            }
            return;
        }

        // else we use Bresenhams line algorithm - CALCULUS yaboii
        let slope = delta_y / delta_x;
        let mut error: f32 = -1.0;
        let mut y = np1.y;
        //println!("new line");
        for x in np1.x..np2.x+1 {

            //println!("x: {}, y: {}, slope: {}, delta_y: {}, delta_x: {}", x, y, slope, delta_y, delta_x);
            let ty = y-(width as i32)/2;
            // A half-assed "solution" for line width.
            for i in 0..width as i32 {
                buffer.set(x,ty+i, color);
            }

            // A simple solution to handle slopes
            error += f32::abs(slope);
            if error >= 0.0 {
                // if slope is positive, y moves upwards, otherwise it might as well go down
                if slope > 0.0 {
                    y += 1;
                }else {
                    if y>0 {
                        y -= 1;
                    }else{
                        break;
                    }
                }
                error -= 1.0;
            }

        }

    }

    pub fn circle_fill(buffer: &mut Buffer, p1: &Point, radius: u32, color: u32) {
        let start_x = p1.x-(radius as i32);
        let end_x = p1.x+(radius as i32);
        let start_y = p1.y-(radius as i32);
        let end_y = p1.y+(radius as i32);

        for x in start_x..end_x+1 {
            for y in start_y..end_y+1 {
                if (x - p1.x) * (x - p1.x) + (y - p1.y) * (y - p1.y) <= radius as i32 * radius as i32 {
                    buffer.set(x, y, color);
                }
            }
        }
    }

    // Very innefficient, but also very pretty :D
    pub fn circle_line(buffer: &mut Buffer, p1: &Point, radius: u32, color: u32, width: u32, bg_color: u32) {
        
        Draw::circle_fill(buffer, p1, radius, color);
        Draw::circle_fill(buffer, p1, radius-width, bg_color);

    }

    pub fn rect_line(buffer: &mut Buffer, p1: &Point, p2: &Point, color: u32, width: u32){
        let mut top_left = Point { x: i32::min(p1.x, p2.x), y: i32::max(p1.y, p2.y) };
        let mut top_right = Point { x: i32::max(p1.x, p2.x), y: i32::max(p1.y, p2.y) };
        let mut bottom_left = Point { x: i32::min(p1.x, p2.x), y: i32::min(p1.y, p2.y) };
        let mut bottom_right = Point { x: i32::max(p1.x, p2.x), y: i32::min(p1.y, p2.y) };

        Draw::line(buffer, &top_left, &top_right, color, width);
        Draw::line(buffer, &top_right, &bottom_right, color, width);
        Draw::line(buffer, &bottom_right, &bottom_left, color, width);
        Draw::line(buffer, &bottom_left, &top_left, color, width);

    }

    pub fn rect_fill(buffer: &mut Buffer, p1: &Point, p2: &Point, color: u32){
        let mut top_left = Point { x: i32::min(p1.x, p2.x), y: i32::max(p1.y, p2.y) };
        let mut top_right = Point { x: i32::max(p1.x, p2.x), y: i32::max(p1.y, p2.y) };
        let mut bottom_left = Point { x: i32::min(p1.x, p2.x), y: i32::min(p1.y, p2.y) };
        let mut bottom_right = Point { x: i32::max(p1.x, p2.x), y: i32::min(p1.y, p2.y) };;

        for x in top_left.x..bottom_right.x+1 {
            for y in bottom_left.y..top_right.y+1 {
                buffer.set(x,y,color);
            }
        }
    }

    
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
}

pub struct Color;

impl Color {
    pub const RED: u32 = 0xFF0000;
    pub const GREEN: u32 = 0x00FF00;
    pub const BLUE: u32 = 0x0000FF;
    pub const BLACK: u32 = 0x000000;
    pub const WHITE: u32 = 0xFFFFFF;
}

pub enum Found {
    OK(u32), NOT
}

impl Found {

}