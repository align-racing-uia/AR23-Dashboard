use std::ops;
pub struct Buffer {
    buffer: Vec<Vec<u32>>,
    pub width: usize,
    pub height: usize,
    clear_color: u32,
}

impl Buffer {
    
    pub fn new(width: usize, height: usize) -> Self {
        Buffer { 
            buffer: vec![vec![Color::WHITE;width];height], 
            width: width, 
            height: height,
            clear_color: Color::WHITE
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![vec![self.clear_color;self.width]; self.height];
    }

    pub fn size(&self) -> (usize, usize) {
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


pub struct Draw;

impl Draw {
    
    pub fn line(buffer: &mut Buffer, p1: &Point, p2: &Point, color: u32, _width: usize) {
        let mut start = Point { x: usize::min(p1.x, p2.x ), y: usize::min(p1.y, p2.y) };
        let mut end   = Point { x: usize::max(p1.x, p2.x ), y: usize::max(p1.y, p2.y) };
        start.within_constraints(&buffer);
        end.within_constraints(&buffer);

        if start.x == end.x {
            for y in start.y..end.y+1 {
                buffer[y][start.x] = color;
            }
        }else if start.y == end.y {
            for x in start.x..end.x+1 {
                buffer[start.y][x] = color;
            }
        }else {
            for x in start.x..end.x+1 {
                let y = start.y + (end.y-start.y)/(end.x-start.x) * (x - start.x);
            }
        }
    }

    pub fn rect_line(buffer: &mut Buffer, p1: &Point, p2: &Point, color: u32){
        let mut top_left = Point { x: usize::min(p1.x, p2.x), y: usize::max(p1.y, p2.y) };
        let mut top_right = Point { x: usize::max(p1.x, p2.x), y: usize::max(p1.y, p2.y) };
        let mut bottom_left = Point { x: usize::min(p1.x, p2.x), y: usize::min(p1.y, p2.y) };
        let mut bottom_right = Point { x: usize::max(p1.x, p2.x), y: usize::min(p1.y, p2.y) };
        top_left.within_constraints(&buffer);
        top_right.within_constraints(&buffer);
        bottom_left.within_constraints(&buffer);
        bottom_right.within_constraints(&buffer);

        Draw::line(buffer, &top_left, &top_right, color, 5);
        Draw::line(buffer, &top_right, &bottom_right, color, 5);
        Draw::line(buffer, &bottom_right, &bottom_left, color, 5);
        Draw::line(buffer, &bottom_left, &top_left, color, 5);

    }

    pub fn rect_fill(buffer: &mut Buffer, p1: &Point, p2: &Point, color: u32){
        let mut top_left = Point { x: usize::min(p1.x, p2.x), y: usize::max(p1.y, p2.y) };
        let mut top_right = Point { x: usize::max(p1.x, p2.x), y: usize::max(p1.y, p2.y) };
        let mut bottom_left = Point { x: usize::min(p1.x, p2.x), y: usize::min(p1.y, p2.y) };
        let mut bottom_right = Point { x: usize::max(p1.x, p2.x), y: usize::min(p1.y, p2.y) };
        top_left.within_constraints(&buffer);
        top_right.within_constraints(&buffer);
        bottom_left.within_constraints(&buffer);
        bottom_right.within_constraints(&buffer);

        for x in top_left.x..bottom_right.x+1 {
            for y in bottom_left.y..top_right.y+1 {
                buffer[y][x] = color;
            }
        }
    }

    
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn within_constraints(&mut self,buffer: &Buffer) {
        if self.x > buffer.width {
            self.x = buffer.width;
        }
        if self.y > buffer.height {
            self.y = buffer.height;
        }
    }
}

pub struct Color;

impl Color {
    pub const RED: u32 = 0xFF0000;
    pub const GREEN: u32 = 0x00FF00;
    pub const BLUE: u32 = 0x0000FF;
    pub const BLACK: u32 = 0x000000;
    pub const WHITE: u32 = 0xFFFFFF;
}

