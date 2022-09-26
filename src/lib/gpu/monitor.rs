use std::thread;
use std::time::Duration;
use sdl2::rect::Point;

use crate::lib::gpu::color::Color;

pub struct Monitor {
    width: u16,
    height: u16,
    data: Vec<Vec<Color>>,
}

impl Monitor {
    pub fn new(w: u16, h: u16) -> Self {
        Monitor {
            width: w,
            height: h,
            data: vec![
                vec![Color::black(); h as usize]; w as usize
            ],
        }
    }

    pub fn write(&mut self, x: u16, y: u16, color: Color) {
        println!("{}:{} -> {}", x, y, color.as_word());
        let x = self.data.get_mut(x as usize).unwrap().insert(y as usize, color);
    }

    pub fn width(&self) -> u16 { self.width }
    pub fn height(&self) -> u16 { self.height }

    pub fn launch(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.present();

        loop {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();
            for x in 0..self.height {
                for y in 0..self.width {
                    let c = self.data.get(x as usize).unwrap().get(y as usize).unwrap();
                    canvas.set_draw_color(sdl2::pixels::Color::RGB(c.r(), c.g(), c.b()));
                    canvas.draw_point(Point::new(x as i32, y as i32));
                }
            }
            canvas.present();

            thread::sleep(Duration::from_micros(50));
        }
    }
}