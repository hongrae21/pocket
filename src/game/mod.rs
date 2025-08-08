use std::time::{Duration, Instant};
use std::cmp::min;

use sdl2::Sdl;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

use crate::math::{collide_circle_and_ellipse, Vec2};
use crate::math::Circle;
use crate::math::Ellipse;

mod ball;

use ball::Ball;

pub struct Game {
    sdl: Sdl,
    cvs: Canvas<Window>,
    elp: Ellipse,
    ball: Ball,
    tick: Instant,
    run: bool
}

pub fn draw_ellipse(cvs: &mut Canvas<Window>, elp: &Ellipse) {
    for i in 0..360 {
        let ang1 = i as f32 * std::f32::consts::PI / 180.0;
        let ang2 = (i + 1) as f32 * std::f32::consts::PI / 180.0;
        let p1 = elp.pos + Vec2 {x: elp.a * f32::cos(ang1), y: elp.b * f32::sin(ang1) };
        let p2 = elp.pos + Vec2 {x: elp.a * f32::cos(ang2), y: elp.b * f32::sin(ang2) };
        cvs.draw_line(p1.to_point(), p2.to_point()).unwrap();
    }
}

pub fn draw_filled_circle(cvs: &mut Canvas<Window>, cir: &Circle) {
    for i in -cir.r as i32..cir.r as i32 + 1 {
        for j in -cir.r as i32..cir.r as i32 + 1 {
            if (i * i + j * j) as f32 <= cir.r * cir.r {
                cvs.draw_point(Point::new(cir.pos.x as i32 + i, cir.pos.y as i32 + j)).unwrap();
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
        let elp = Ellipse {pos: Vec2 {x: 400.0, y: 300.0}, a: 250.0, b: 150.0};
        let ball = Ball {
            obj: Circle {pos: Vec2 {x: 600.0, y: 300.0}, r: 10.0},
            dir: Vec2 {x: f32::sqrt(2.0) / 2.0, y: f32::sqrt(2.0)},
            sped: 50.0
        };
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
        let fps = 120;
        while (Instant::now() - self.tick) > Duration::new(0, 1_000_000_000u32 / fps) {}

        let dt = self.get_dt();
        self.ball.update(dt);

        let coll = collide_circle_and_ellipse(&self.ball.obj, &self.elp);
        if coll > 0.0{
            let nang = coll + std::f32::consts::PI / 2.0;
            let norm = Vec2 {x: f32::cos(nang), y: f32::sin(nang)};
            self.ball.dir = Vec2::refl(self.ball.dir, norm);
        }

    }

    fn output(&mut self) {
        self.cvs.set_draw_color(Color::WHITE);
        self.cvs.clear();


        // draw ellipse
        self.cvs.set_draw_color(Color::BLACK);
        draw_ellipse(&mut self.cvs, &self.elp);

        // draw ball
        self.cvs.set_draw_color(Color::GRAY);
        draw_filled_circle(&mut self.cvs, &self.ball.obj);

        // mark focus of ellipse
        self.cvs.set_draw_color(Color::GREEN);
        draw_filled_circle(&mut self.cvs, &Circle {pos: self.elp.focus().0, r: 3.0});
        draw_filled_circle(&mut self.cvs, &Circle {pos: self.elp.focus().1, r: 3.0});

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
