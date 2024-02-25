use macroquad::prelude::*;
use macroquad::rand::*;

const SHIP_HEIGHT: f32 = 25.0;
const SHIP_BASE: f32 = 12.5;
const BARBELL_WIDTH: f32 = 40.0;
const ROTATION: f32 = 10.0;

struct Ship {
    middle: Vec2,
    rot: f32,
    vel: Vec2,
}

impl Ship {
    const HEIGHT: Vec2 = Vec2::new(0.0, -SHIP_HEIGHT / 2.0);
    const RIGHT: Vec2 = Vec2::new(SHIP_BASE / 2.0, SHIP_HEIGHT / 2.0);
    const LEFT: Vec2 = Vec2::new(-SHIP_BASE / 2.0, SHIP_HEIGHT / 2.0);
    fn draw(&self) {
        if self.middle.x < SHIP_HEIGHT / 2.0 || self.middle.x > screen_width() - (SHIP_HEIGHT / 2.0)
        {
            return;
        }
        if self.middle.y < SHIP_HEIGHT / 2.0
            || self.middle.y > screen_height() - (SHIP_HEIGHT / 2.0)
        {
            return;
        }
        let rotation = Mat2::from_angle(self.rot);
        let top = (rotation * Self::HEIGHT) + self.middle;
        let right = (rotation * Self::RIGHT) + self.middle;
        let left = (rotation * Self::LEFT) + self.middle;
        draw_triangle(top, left, right, BLUE);
    }

    fn rotate(&mut self, angle: f32) {
        self.rot += angle;
        // let (sin, cos) = angle.sin_cos();
    }

    fn vroom(&mut self) {
        self.middle += self.vel;
        wrap(&mut self.middle);
    }

    fn accelerate(&mut self, delta: Vec2) {
        self.vel += delta;
        if self.vel.length() > 10.0 {
            self.vel = self.vel.normalize() * 10.0;
        }
    }
}

struct Barbell {
    middle: Vec2,
    rot: f32,
    vel: Vec2,
}

impl Barbell {
    const LEFT: Vec2 = Vec2::new(-BARBELL_WIDTH / 2.0, 0.0);
    const RIGHT: Vec2 = Vec2::new(BARBELL_WIDTH / 2.0, 0.0);
    fn draw(&self) {
        let rotation = Mat2::from_angle(self.rot);
        let left = (rotation * Self::LEFT) + self.middle;
        let right = (rotation * Self::RIGHT) + self.middle;
        draw_line(left.x, left.y, right.x, right.y, 3.0, RED);
    }

    fn rotate(&mut self, angle: f32) {
        self.rot += angle;
        // let (sin, cos) = angle.sin_cos();
    }

    fn vroom(&mut self) {
        self.middle += self.vel;
        wrap(&mut self.middle);
    }
}

fn wrap(v: &mut Vec2) {
    if v[0] >= screen_width() {
        v[0] = 0.0;
    }
    if v[0] < 0.0 {
        v[0] = screen_width();
    }
    if v[1] >= screen_height() {
        v[1] = 0.0;
    }
    if v[1] < 0.0 {
        v[1] = screen_height();
    }
}

#[macroquad::main("InputKeys")]
async fn main() {
    let mid = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut ship = Ship {
        middle: mid,
        rot: 0.0,
        vel: Vec2::ZERO,
    };
    let mut barbell_middle = Vec2::new(rand() as f32, rand() as f32);
    wrap(&mut barbell_middle);
    let mut barbell = Barbell {
        middle: barbell_middle,
        rot: rand() as f32,
        vel: Vec2::new(4.0, 4.0),
    };
    let rotation = ROTATION.to_radians();
    loop {
        clear_background(LIGHTGRAY);
        if is_key_down(KeyCode::Right) {
            ship.rotate(rotation);
        }
        if is_key_down(KeyCode::Left) {
            ship.rotate(-rotation);
        }
        if is_key_down(KeyCode::Down) {
            // TODO
        }
        if is_key_down(KeyCode::Up) {
            let orientation = ship.rot + (std::f32::consts::PI / 2.0);
            // println!("o {}", orientation.to_degrees() % 360.0);
            let acc = Vec2::new(-orientation.cos(), -orientation.sin()) * 1.0;
            ship.accelerate(acc);
        }
        // let x: u32 = (std::f32::consts::PI / 8.0) as u32;
        barbell.rotate((rand() as f32) % (std::f32::consts::PI / 128.0));
        barbell.vroom();
        barbell.draw();
        ship.vroom();
        ship.draw();
        draw_text("barbells", 20.0, 20.0, 20.0, BLUE);
        next_frame().await
    }
}
