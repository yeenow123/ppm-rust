extern crate sdl2;

mod ppm;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::rect::Point;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut buffer = Vec::new();
    let mut f = File::open(filename)?;
    f.read_to_end(&mut buffer)?;

    let format = ppm::PPM::new(buffer);

    let width = format.width;
    let height = format.height;
    let total_size = (width * height) as usize;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(filename, width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();

    for i in 0..total_size {
        let curr_pixel = &format.data[i];
        canvas.set_draw_color(pixels::Color::RGB(curr_pixel.0, curr_pixel.1, curr_pixel.2));
        canvas
            .draw_point(Point::new(
                (i % width as usize) as i32,
                (i / width as usize) as i32,
            ))
            .expect("Could not draw point");
    }
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}
