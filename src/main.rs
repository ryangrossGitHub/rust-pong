use macroquad::prelude::*;

mod direction;
use crate::direction::Direction;

mod ball;
use crate::ball::Ball;

mod players;
use crate::players::human_player::HumanPlayer;
use crate::players::bot_player::BotPlayer;

mod score_board;

#[macroquad::main("InputKeys", "Time-based movement")]
async fn main() {
    let mut delta_time: f32;
    let mut screen_width_val: f32 = screen_width();
    let mut screen_height_val: f32 = screen_height();

    let mut player1: HumanPlayer = HumanPlayer::new(screen_height_val / 2.0);
    let mut bot: BotPlayer = BotPlayer::new(screen_height_val / 2.0, screen_width_val);

    let mut ball: Ball = Ball::new(
        screen_width_val / 2.0,
        screen_height_val / 2.0,
        300.0,
        Direction::Right,
        Direction::Down
    );

    loop {
        delta_time = get_frame_time();
        if is_key_down(KeyCode::Escape) {
            break;
        } 

        screen_width_val = screen_width();
        screen_height_val = screen_height();

        clear_background(BLACK);

        player1.update(screen_height_val, delta_time);
        bot.update_x(screen_width_val);
        bot.update_y(screen_height_val, ball.y(), delta_time);
        ball.update(screen_height_val, &mut player1, &mut bot, delta_time);

        if ball.x() <= 0.0 {
            bot.player().increment_score();
            ball.reset(screen_width_val / 2.0, screen_height_val / 2.0);
        } else if ball.x() >= screen_width_val {
            player1.player().increment_score();
            ball.reset(screen_width_val / 2.0, screen_height_val / 2.0);
        }

        draw_rectangle(players::human_player::X, player1.player().y(), players::player::WIDTH, 
            players::player::HEIGHT, WHITE);
        draw_rectangle(bot.x(), bot.player().y(), players::player::WIDTH, players::player::HEIGHT,
             WHITE);
        draw_circle(ball.x(), ball.y(), ball::RADIUS, WHITE);
        draw_text(player1.player().score().to_string().as_str(), score_board::SCORE_TEXT_X_OFFSET, 
            score_board::SCORE_TEXT_Y_OFFSET, score_board::SCORE_FONT_SIZE, 
            score_board::SCORE_FONT_COLOR);
        draw_text(bot.player().score().to_string().as_str(), 
            screen_width_val - score_board::SCORE_TEXT_X_OFFSET, score_board::SCORE_TEXT_Y_OFFSET, 
            score_board::SCORE_FONT_SIZE, score_board::SCORE_FONT_COLOR);
        next_frame().await
    }
}