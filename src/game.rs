use std::time::{Duration, Instant};
use std::cmp::min;

use sdl2::Sdl;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

use crate::math::Ellipse;
use crate::math::Circle;

pub struct Game {
    sdl: Sdl,
    cvs: Canvas<Window>,
    elp: Ellipse,
    ball: Circle,
    tick: Instant,
    run: bool
}

pub fn draw_ellipse(cvs: &mut Canvas<Window>, elp: &Ellipse) {
    for i in 0..360 {
        let ang1 = i as f32 * std::f32::consts::PI / 180.0;
        let ang2 = (i + 1) as f32 * std::f32::consts::PI / 180.0;
        let x1 = (elp.x + elp.a * f32::cos(ang1)) as i32;
        let y1 = (elp.y + elp.b * f32::sin(ang1)) as i32;
        let x2 = (elp.x + elp.a * f32::cos(ang2)) as i32;
        let y2 = (elp.y + elp.b * f32::sin(ang2)) as i32;
        cvs.draw_line(Point::new(x1, y1), Point::new(x2, y2)).unwrap();
    }
}

pub fn draw_filled_circle(cvs: &mut Canvas<Window>, cir: &Circle) {
    for i in -cir.r as i32..cir.r as i32 + 1 {
        for j in -cir.r as i32..cir.r as i32 + 1 {
            if (i * i + j * j) as f32 <= cir.r * cir.r {
                cvs.draw_point(Point::new(cir.x as i32 + i, cir.y as i32 + j)).unwrap();
            }
        }
    }
}


impl Game {
    pub fn new() -> Game {
        let sdl = sdl2::init().unwrap();
        let vid = sdl.video().unwrap();
        let win = vid.window("pocket", 800, 600)
        .position_centered()
        .build()
        .unwrap();
        let cvs = win.into_canvas().build().unwrap();
        let elp = Ellipse {x: 400.0, y: 300.0, a: 250.0, b: 150.0};
        let ball  = Circle {x: 400.0, y: 300.0, r: 10.0};
        Game {
            sdl: sdl,
            cvs: cvs,
            elp: elp,
            ball: ball,
            tick: Instant::now(),
            run: true
        }
    }

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


        // draw ellipse
        self.cvs.set_draw_color(Color::BLACK);
        draw_ellipse(&mut self.cvs, &self.elp);

        // draw ball
        self.cvs.set_draw_color(Color::GRAY);
        draw_filled_circle(&mut self.cvs, &self.ball);

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
