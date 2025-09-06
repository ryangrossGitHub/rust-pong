use crate::players::player::{self, Player};

const X_OFFSET: f32 = 20.0;
const SPEED: f32 = 0.4;

pub struct BotPlayer {
    player: Player,
    x: f32,
}

impl BotPlayer {
    pub fn new(y: f32, screen_width: f32) -> BotPlayer {
        BotPlayer { player: Player::new(y), x: screen_width - X_OFFSET }
    }

    pub fn player(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn update_x(&mut self, screen_width: f32) {
        self.x = screen_width - X_OFFSET;
    }

    pub fn update_y(&mut self, screen_height: f32, ball_y: f32) {
        if self.player.y() < screen_height - player::HEIGHT && 
            ball_y > self.player.y() + player::HEIGHT / 2.0 {
                self.player.add_y(SPEED);
        } else if self.player.y() > 0.0 && 
            ball_y < self.player.y() + player::HEIGHT / 2.0 {
                self.player.subtract_y(SPEED);
        }
    }
}