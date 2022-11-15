use minifb::*;

pub mod helper;
use helper::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 480;


fn main(){
    let mut buffer: Buffer = Buffer::new(WIDTH as u32, HEIGHT as u32);
    //let mut buffer: Vec<u32> = vec![];
    let win_ops = WindowOptions {
        borderless: true,
        ..Default::default()
    };

    // for actually displaying stuff on windows, and in a desktop enviroment
    let mut window = Window::new(
        "AR23",
        WIDTH,
        HEIGHT,
        win_ops
        
    ).unwrap_or_else(|e|{
        panic!("{}",e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    let p1 = Point { x: 300, y: 100 };
    let p2 = Point { x: 500, y: 200 };

    
    let mut i: u16 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        i += 1;
        let mut color = Color::BLUE;
        //buffer.clear();
        if i < 10 {
            color = Color::RED;
        } else if i<20 {
            color = Color::GREEN;
        } else if i>30 {
            i = 0;
        }

        let clear_color = buffer.clear_color;
        Draw::circle_line(&mut buffer, &p1, 100, color, 10, clear_color);
        Draw::rect_line(&mut buffer, &p1, &p2, color, 5);
        window.update_with_buffer(&buffer.to_buffer(), WIDTH, HEIGHT).unwrap();
    }

}


