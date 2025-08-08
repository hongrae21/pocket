use std::time::{Duration, Instant};
use std::thread::sleep;
use std::cmp::min;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

pub struct Game {
    sdl: Sdl,
    cvs: Canvas<Window>,
    tick: Instant,
    run: bool
}

pub fn init_game() -> Game {
    let sdl = sdl2::init().unwrap();
    let vid = sdl.video().unwrap();
    let win = vid.window("pocket", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let cvs = win.into_canvas().build().unwrap();
    Game {
        sdl: sdl,
        cvs: cvs,
        tick: Instant::now(),
        run: true
    }
}

impl Game {
    fn input(&mut self) {
        let mut pump = self.sdl.event_pump().unwrap();
        for e in pump.poll_iter() {
            match e {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    self.run = false;
                }
                _ => {}
            }
        }

    }

    fn get_dt(&mut self) -> Duration {
        let now = Instant::now();
        let mut dt = now - self.tick;
        self.tick = now;
        dt = min(dt, Duration::new(0, 1_000_000_000u32 / 2));
        dt
    }

    fn update(&mut self) {
        let fps = 60;
        while (Instant::now() - self.tick) > Duration::new(0, 1_000_000_000u32 / fps) {}

        let dt = self.get_dt();

    }

    fn output(&mut self) {
        self.cvs.set_draw_color(Color::WHITE);
        self.cvs.clear();
        self.cvs.present();
    }

    pub fn run(&mut self) {
        while self.run {
            self.input();
            self.update();
            self.output();
        }
    }
}
