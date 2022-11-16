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


    let p1 = Point { x: 300, y: 300 };
    let p2 = Point { x: 500, y: 200 };

    let mut txt = Text::new(&mut buffer, p1, "Hello, World", 40.0, Color::RED);

    let mut i: u16 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        i += 1;
        buffer.clear();
        txt.update_text(&buffer, i.to_string().as_str());
        txt.draw(&mut buffer);
        window.update_with_buffer(&buffer.to_buffer(), WIDTH, HEIGHT).unwrap();
    }

}


