use macroquad::prelude::*;
use macroquad::rand::*;

const PI: f32 = std::f32::consts::PI;
const SHIP_HEIGHT: f32 = 25.0;
const SHIP_BASE: f32 = 12.5;
const BARBELL_WIDTH: f32 = 80.0;
const ROTATION: f32 = 10.0;
const BARBELL_COLOR: Color = DARKPURPLE;
const SHIP_COLOR: Color = LIME;

struct Ship {
    middle: Vec2,
    rot: f32,
    vel: Vec2,
    // Loaded
    top: Vec2,
    right: Vec2,
    left: Vec2,
}

// Check if triangle and bell lines intersect
fn intersect(line_1a: &Vec2, line_1b: &Vec2, line_2a: &Vec2, line_2b: &Vec2) -> bool {
    let x1 = line_1a.x;
    let x2 = line_1b.x;
    let x3 = line_2a.x;
    let x4 = line_2b.x;
    let y1 = line_1a.y;
    let y2 = line_1b.y;
    let y3 = line_2a.y;
    let y4 = line_2b.y;
    let u_a = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
    let u_b = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
    (0.0..=1.0).contains(&u_a) && (0.0..=1.0).contains(&u_b)
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
        draw_triangle(self.top, self.left, self.right, SHIP_COLOR);
    }

    // Call after reloading both
    fn hits_bells(&self, barbell: &Barbell) -> bool {
        let v = [self.left, self.right, self.top];
        for (i, point1) in v.iter().enumerate() {
            for (j, point2) in v.iter().enumerate() {
                if i == j {
                    continue;
                }
                if intersect(
                    point1,
                    point2,
                    &barbell.right_bell_top,
                    &barbell.right_bell_bot,
                ) {
                    return true;
                }
                if intersect(
                    point1,
                    point2,
                    &barbell.left_bell_top,
                    &barbell.left_bell_bot,
                ) {
                    return true;
                }
            }
        }
        false
    }

    fn hits_center(&self, barbell: &Barbell) -> bool {
        let v = [self.left, self.right, self.top];
        for (i, point1) in v.iter().enumerate() {
            for (j, point2) in v.iter().enumerate() {
                if i == j {
                    continue;
                }
                if intersect(point1, point2, &barbell.right, &barbell.left) {
                    return true;
                }
            }
        }
        false
    }

    fn reload(&mut self) {
        let rotation = Mat2::from_angle(self.rot);
        self.top = (rotation * Self::HEIGHT) + self.middle;
        self.right = (rotation * Self::RIGHT) + self.middle;
        self.left = (rotation * Self::LEFT) + self.middle;
    }

    fn rotate(&mut self, angle: f32) {
        self.rot += angle;
        // let (sin, cos) = angle.sin_cos();
    }

    fn vroom(&mut self) {
        self.middle += self.vel;
        wrap(&mut self.middle);
        self.vel *= 0.90;
        self.reload()
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
    // Loaded
    left: Vec2,
    right: Vec2,
    left_bell_top: Vec2,
    left_bell_bot: Vec2,
    right_bell_top: Vec2,
    right_bell_bot: Vec2,
}

impl Barbell {
    const LEFT: Vec2 = Vec2::new(-BARBELL_WIDTH / 2.0, 0.0);
    const RIGHT: Vec2 = Vec2::new(BARBELL_WIDTH / 2.0, 0.0);
    const LEFT_BELL_BOT: Vec2 = Vec2::new(-BARBELL_WIDTH / 2.0, BARBELL_WIDTH / 6.0);
    const LEFT_BELL_TOP: Vec2 = Vec2::new(-BARBELL_WIDTH / 2.0, -BARBELL_WIDTH / 6.0);
    const RIGHT_BELL_BOT: Vec2 = Vec2::new(BARBELL_WIDTH / 2.0, BARBELL_WIDTH / 6.0);
    const RIGHT_BELL_TOP: Vec2 = Vec2::new(BARBELL_WIDTH / 2.0, -BARBELL_WIDTH / 6.0);

    fn draw(&self) {
        // Draw bar
        draw_line(
            self.left.x,
            self.left.y,
            self.right.x,
            self.right.y,
            3.0,
            BARBELL_COLOR,
        );
        // Draw self.left bell
        draw_line(
            self.left.x,
            self.left.y,
            self.left_bell_top.x,
            self.left_bell_top.y,
            3.0,
            BARBELL_COLOR,
        );
        draw_line(
            self.left.x,
            self.left.y,
            self.left_bell_bot.x,
            self.left_bell_bot.y,
            3.0,
            BARBELL_COLOR,
        );
        // Draw self.right bell
        draw_line(
            self.right.x,
            self.right.y,
            self.right_bell_top.x,
            self.right_bell_top.y,
            3.0,
            BARBELL_COLOR,
        );
        draw_line(
            self.right.x,
            self.right.y,
            self.right_bell_bot.x,
            self.right_bell_bot.y,
            3.0,
            BARBELL_COLOR,
        );
    }

    fn reload(&mut self) {
        let rotation = Mat2::from_angle(self.rot);
        // Rotate bar
        self.left = (rotation * Self::LEFT) + self.middle;
        self.right = (rotation * Self::RIGHT) + self.middle;
        // Rotate left bell
        self.left_bell_bot = (rotation * Self::LEFT_BELL_BOT) + self.middle;
        self.left_bell_top = (rotation * Self::LEFT_BELL_TOP) + self.middle;
        // Rotate right bell
        self.right_bell_bot = (rotation * Self::RIGHT_BELL_BOT) + self.middle;
        self.right_bell_top = (rotation * Self::RIGHT_BELL_TOP) + self.middle;
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
        self.reload();
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
        // Loaded
        top: Vec2::ZERO,
        right: Vec2::ZERO,
        left: Vec2::ZERO,
    };
    let mut barbell_middle = Vec2::new(rand() as f32, rand() as f32);
    wrap(&mut barbell_middle);
    let rx = (rand() as f32) % 5.0;
    let ry = (rand() as f32) % 5.0;
    let mut barbell = Barbell {
        middle: barbell_middle,
        rot: rand() as f32,
        vel: Vec2::new(rx, ry),
        clockwise: true,
        // Loaded
        right: Vec2::ZERO,
        left: Vec2::ZERO,
        left_bell_top: Vec2::ZERO,
        left_bell_bot: Vec2::ZERO,
        right_bell_top: Vec2::ZERO,
        right_bell_bot: Vec2::ZERO,
    };
    let rotation = ROTATION.to_radians();
    loop {
        clear_background(WHITE);
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
        ship.vroom();

        // check if collision
        if ship.hits_bells(&barbell) {
            draw_text("NO", 50.0, 50.0, 50.0, RED)
        }
        // check if collision
        if ship.hits_center(&barbell) {
            draw_text("YES", 70.0, 70.0, 70.0, GREEN)
        }

        ship.draw();
        barbell.draw();
        draw_text("barbells", 20.0, 20.0, 20.0, BLUE);
        next_frame().await
    }
}
