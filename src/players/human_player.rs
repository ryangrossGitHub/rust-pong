use macroquad::input::is_key_down;
use macroquad::prelude::KeyCode;

use crate::players::player;
use crate::players::player::Player;

const SPEED: f32 = 300.0;
pub const X: f32 = 10.0;

pub struct HumanPlayer {
    player: Player,
}

impl HumanPlayer {
    pub fn new(y: f32) -> HumanPlayer {
        HumanPlayer { player: Player::new(y) }
    }

    pub fn player(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn update(&mut self, screen_height: f32, delta_time: f32) {
        if self.player.y() < screen_height - player::HEIGHT && is_key_down(KeyCode::Down) {
            self.player.add_y(SPEED * delta_time);
        } else if self.player.y() > 0.0 && is_key_down(KeyCode::Up) {
            self.player.subtract_y(SPEED * delta_time);
        }
    }
}