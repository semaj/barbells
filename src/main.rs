use macroquad::prelude::*;
use macroquad::rand::*;

const PI: f32 = std::f32::consts::PI;
const SHIP_HEIGHT: f32 = 25.0;
const SHIP_BASE: f32 = 12.5;
const BARBELL_WIDTH: f32 = 80.0;
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
        // TODO: Deceleration
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
    clockwise: bool,
}

impl Barbell {
    const LEFT: Vec2 = Vec2::new(-BARBELL_WIDTH / 2.0, 0.0);
    const RIGHT: Vec2 = Vec2::new(BARBELL_WIDTH / 2.0, 0.0);
    const LEFT_BELL_BOT: Vec2 = Vec2::new(-BARBELL_WIDTH / 2.0, BARBELL_WIDTH / 6.0);
    const LEFT_BELL_TOP: Vec2 = Vec2::new(-BARBELL_WIDTH / 2.0, -BARBELL_WIDTH / 6.0);
    const RIGHT_BELL_BOT: Vec2 = Vec2::new(BARBELL_WIDTH / 2.0, BARBELL_WIDTH / 6.0);
    const RIGHT_BELL_TOP: Vec2 = Vec2::new(BARBELL_WIDTH / 2.0, -BARBELL_WIDTH / 6.0);

    fn draw(&self) {
        let rotation = Mat2::from_angle(self.rot);
        // Rotate bar
        let left = (rotation * Self::LEFT) + self.middle;
        let right = (rotation * Self::RIGHT) + self.middle;
        // Rotate left bell
        let left_bell_bot = (rotation * Self::LEFT_BELL_BOT) + self.middle;
        let left_bell_top = (rotation * Self::LEFT_BELL_TOP) + self.middle;
        // Rotate right bell
        let right_bell_bot = (rotation * Self::RIGHT_BELL_BOT) + self.middle;
        let right_bell_top = (rotation * Self::RIGHT_BELL_TOP) + self.middle;
        // Draw bar
        draw_line(left.x, left.y, right.x, right.y, 3.0, RED);
        // Draw left bell
        draw_line(left.x, left.y, left_bell_top.x, left_bell_top.y, 3.0, RED);
        draw_line(left.x, left.y, left_bell_bot.x, left_bell_bot.y, 3.0, RED);
        // Draw right bell
        draw_line(
            right.x,
            right.y,
            right_bell_top.x,
            right_bell_top.y,
            3.0,
            RED,
        );
        draw_line(
            right.x,
            right.y,
            right_bell_bot.x,
            right_bell_bot.y,
            3.0,
            RED,
        );
    }

    fn rotate(&mut self) {
        if (rand() % 1000) > 990 {
            self.clockwise = !self.clockwise;
        }
        // let rotation = (rand() as f32) % (PI / 120.0);
        let rotation = PI / 300.0;
        if self.clockwise {
            self.rot += rotation;
        } else {
            self.rot -= rotation;
        }
        self.rot %= PI * 2.0;
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
        clockwise: true,
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
            let orientation = ship.rot + (PI / 2.0);
            // println!("o {}", orientation.to_degrees() % 360.0);
            let acc = Vec2::new(-orientation.cos(), -orientation.sin()) * 1.0;
            ship.accelerate(acc);
        }
        barbell.rotate();
        barbell.vroom();
        barbell.draw();
        ship.vroom();
        ship.draw();
        draw_text("barbells", 20.0, 20.0, 20.0, BLUE);
        next_frame().await
    }
}
