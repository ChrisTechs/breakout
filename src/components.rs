use std::f32::consts::PI;
use macroquad::prelude::*;

const DEG_TO_RAD: f32 = PI/180f32;

pub struct Ball {
    pub circle: Circle,
    pub vel: Vec2,
    pub ball_radius: f32,
    ball_speed: f32,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        let ball_radius = (screen_width() + screen_height()) * 0.02f32;
        let ball_speed = screen_height() * 0.84;

        Self {
            circle: Circle::new(pos.x, pos.y, ball_radius),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
            ball_radius,
            ball_speed,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.circle.move_to(vec2(
            self.circle.x + self.vel.x * dt * self.ball_speed,
            self.circle.y + self.vel.y * dt * self.ball_speed
        ));

        if self.circle.x - self.circle.r < 0f32 {
            self.vel.x = 1f32;
        }
        if self.circle.x + self.circle.r > screen_width() {
            self.vel.x = -1f32;
        }

        if self.circle.y - self.circle.r < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, WHITE)
    }
}

#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    SpawnBall,
    FreezePlayer,
}

pub struct Block {
    pub rect: Rect,
    pub lives: i32,
    pub score: i32,
    pub block_type: BlockType,
    pub block_size: Vec2,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        let block_size = vec2(screen_width() / 100f32 * 10f32, screen_height() / 100f32 * 5f32);

        let lives = rand::gen_range(1, 4);

        let block_type = match rand::gen_range(1, 11) {
            3 => match rand::gen_range(1, 4) {
                2 => BlockType::FreezePlayer,
                _ => BlockType::Regular
            },
            2 => BlockType::SpawnBall,
            _ => BlockType::Regular,
        };

        Self {
            rect: Rect::new(pos.x, pos.y, block_size.x, block_size.y),
            score: lives,
            lives,
            block_type,
            block_size,
        }
    }

    pub fn draw(&self) {
        let colour: Color = match self.block_type {

            BlockType::Regular => match self.lives {
                3 => GREEN,
                2 => YELLOW,
                1 => RED,
                _ => DARKGRAY
            }

            BlockType::FreezePlayer => match self.lives {
                3 => DARKBLUE,
                2 => BLUE,
                1 => Color::new(0.0, 1.0, 1.0, 1.0),
                _ => DARKGRAY
            }

            _ => match self.lives {
                3 => DARKGREEN,
                2 => ORANGE,
                1 => Color::new(0.5451, 0.0, 0.0, 1.00),
                _ => DARKGRAY
            }

        };

        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, colour);
    }
}

pub struct Player {
    pub rect: Rect,
    pub player_size: Vec2,
    player_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        let player_size = vec2(screen_width() / 100f32 * 15f32, screen_height() / 100f32 * 5f32);
        let player_speed = screen_width() * 0.95;
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - player_size.x * 0.5f32,
                screen_height() - 100f32,
                player_size.x,
                player_size.y
            ),
            player_size,
            player_speed,
        }
    }

    pub fn update(&mut self, dt: f32, font: Font, frozen: &f64) {

        let now = &(miniquad::date::now() * 1000f64);

        if now < frozen {
            draw_title_text("Frozen", font, BLUE);
            return;
        }

        let x_move = match (
            is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
            is_key_down(KeyCode::D) || is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32
        };

        self.rect.x += x_move * dt * self.player_speed;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK);
    }
}

pub enum GameState {
    Menu,
    Game,
    LostLife,
    Died,
    Win,
}

pub fn resolve_collision(ball: &mut Circle, vel: &mut Vec2, object: &Rect) -> bool {

    let ball_rect = Rect::new(ball.x - ball.r, ball.y - ball.r, ball.r * 1.8, ball.r * 1.8);

    let intersection = match ball_rect.intersect(*object) {
        Some(intersection) => intersection,
        None => return false,
    };

    let object_centre = object.point() + object.size() * 0.5f32;

    let ball_centre = ball.point();

    let signum = (object_centre - ball_centre).signum();

    if vel.y > 0.8 {
        if ball_centre.x < object_centre.x {
            rotate(vel, 15f32 * DEG_TO_RAD);
        }else {
            rotate(vel, -15f32 * DEG_TO_RAD);
        };
    }

    match intersection.w > intersection.h {
        true => {
            ball.y -= signum.y * intersection.h;
            match signum.y > 0f32 {
                true => vel.y = -vel.y.abs(),
                false => vel.y = vel.y.abs(),
            }
        }
        false => {
            ball.x -= signum.x * intersection.w;
            match signum.x < 0f32 {
                true => vel.x = vel.x.abs(),
                false => vel.x = -vel.x.abs(),
            }
        }
    }

    true
}

fn rotate(vec: &mut Vec2, radians: f32) {
    let c_ang = radians.cos();
    let s_ang = radians.sin();

    *vec = vec2(c_ang*vec.x - s_ang*vec.y, s_ang*vec.x + c_ang*vec.y).normalize();
}

pub fn draw_title_text(text: &str, font: Font, colour: Color) {

    let dims = measure_text(text, Some(font), 50u16, 1f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.6f32 - dims.height * 0.5f32,
        TextParams { font, font_size: 50u16, color: colour, ..Default::default() }
    )
}