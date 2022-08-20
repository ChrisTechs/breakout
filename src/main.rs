mod components;

use components::{Ball, Block, Player, GameState, resolve_collision, draw_title_text};

use macroquad::prelude::*;
use crate::components::BlockType;
use crate::rand::srand;

fn create_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
) {

    let time = get_time();

    srand((time * 10000000000000000f64 - time * 1000000000000000f64) as u64);

    *player_lives = rand::gen_range(4, 7);

    blocks.clear();
    balls.clear();

    let block_size = Block::new(vec2(0f32, 0f32)).block_size;

    let padding = block_size.x * 0.05;
    let total_block_size = block_size + vec2(padding, padding);
    let (width, height) = ((screen_width()/total_block_size.x * 0.8) as i32, (screen_height()/total_block_size.y * 0.4) as i32);
    let board_start_pos = vec2((screen_width() - total_block_size.x * (width as f32)) * 0.5f32, 50f32);

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }

    *score = 0;

    for _i in 0..*player_lives {
        balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.55f32)));
    }
}

#[macroquad::main("breakout")]
async fn main() {

    let font = load_ttf_font_from_bytes(include_bytes!("res/Silkscreen-Regular.ttf")).unwrap();

    let mut game_state = GameState::Menu;

    let mut score: i32 = 0;
    let mut player_lives = 0;
    let mut frozen = 0f64;

    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();

    create_game(&mut score, &mut player_lives, &mut blocks, &mut balls);

    loop {

        clear_background(DARKBROWN);
        player.draw();

        for block in blocks.iter() {
            block.draw()
        }
        for ball in balls.iter() {
            ball.draw()
        }

        let mut to_add = vec![];

        match game_state {
            GameState::Menu => {
                draw_title_text("press space to start", font, BLACK);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {

                player.update(get_frame_time(), font, &frozen);

                let balls_len = balls.len();
                let mut was_last_ball = balls_len == 1;

                for i in 0..balls.len() {
                match balls.get_mut(i) {
                    None => {}
                    Some(ball) => {

                        ball.update(get_frame_time());

                        resolve_collision(&mut ball.circle, &mut ball.vel, &player.rect);

                        for block_i in 0..blocks.len() {
                        match blocks.get_mut(block_i) {
                            None => {continue}
                            Some(block) => {
                                if resolve_collision(&mut ball.circle, &mut ball.vel, &block.rect) {
                                    block.lives -= 1;

                                    if block.lives <= 0 {
                                        score += block.score;
                                        if block.block_type == BlockType::SpawnBall {
                                            let mut ball = Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.55f32));
                                            ball.circle.x = block.rect.x;
                                            ball.circle.y = block.rect.y;
                                            to_add.push(ball);
                                        }
                                        if block.block_type == BlockType::FreezePlayer {
                                            let mut ball = Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.55f32));
                                            ball.circle.x = block.rect.x;
                                            ball.circle.y = block.rect.y;

                                            let time = miniquad::date::now() * 1000f64;

                                            frozen = time + 1400f64;

                                            to_add.push(ball);
                                        }
                                        blocks.remove(block_i);
                                    }
                                }
                            }
                        }
                        }

                        if ball.circle.y > (screen_height() - player.player_size.y * 1.1f32) {
                            balls.remove(i);
                        }

                    }
                }
                }

                if !to_add.is_empty() {
                    was_last_ball = false;
                }

                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && was_last_ball {

                    player_lives -= 1;

                    player = Player::new();

                    for _i in 0..player_lives {
                        balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.55f32)));
                    }

                    game_state = GameState::LostLife;
                }

                for ball in to_add.into_iter() {
                    balls.push(ball)
                }

                let score_text = format!("score: {}", score);
                let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1f32);

                draw_text_ex(
                    &score_text,
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    40.0,
                    TextParams {font, font_size: 30u16, color: BLACK, ..Default::default()}
                );

                draw_text_ex(
                    &format!("lives: {}", player_lives),
                    30.0,
                    40.0,
                    TextParams {font, font_size: 30u16, color: BLACK, ..Default::default()}
                );
                if blocks.is_empty() {
                    game_state = GameState::Win;
                }else if player_lives <= 0 {
                    game_state = GameState::Died;
                }
            }
            GameState::LostLife => {
                draw_title_text("press space to continue", font, BLACK);

                let score_text = format!("score: {}", score);
                let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1f32);

                draw_text_ex(
                    &score_text,
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    40.0,
                    TextParams {font, font_size: 30u16, color: BLACK, ..Default::default()}
                );

                draw_text_ex(
                    &format!("lives: {}", player_lives),
                    30.0,
                    40.0,
                    TextParams {font, font_size: 30u16, color: BLACK, ..Default::default()}
                );

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game
                }
            }
            GameState::Died => {
                draw_title_text(&format!("you died! {} score", score), font, BLACK);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;

                    create_game(&mut score, &mut player_lives, &mut blocks, &mut balls);

                }
            }
            GameState::Win => {
                draw_title_text(&format!("you win! {} score", score), font, BLACK);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;

                    create_game(&mut score, &mut player_lives, &mut blocks, &mut balls);
                }
            }
        }

        next_frame().await
    }
}