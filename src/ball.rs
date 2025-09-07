use crate::direction::Direction;
use crate::players::human_player::{self, HumanPlayer};
use crate::players::bot_player::BotPlayer;
use crate::players::player;

pub struct Ball {
    x: f32,
    y: f32,
    speed: f32,
    dir_x: Direction,
    dir_y: Direction
}

pub const RADIUS: f32 = 8.0;

impl Ball {
    pub fn new(x: f32, y: f32, speed: f32, dir_x: Direction, dir_y: Direction) -> Ball {
        Ball { x, y, speed, dir_x, dir_y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.dir_x = Direction::Right;
        self.dir_y = Direction::Down;
    }

    pub fn update(&mut self, screen_height: f32, player1: &mut HumanPlayer, bot: &mut BotPlayer, delta_time: f32) {
        self.update_dir_x(player1, bot);
        self.update_dir_y(screen_height);
        self.update_x(delta_time);
        self.update_y(delta_time);
    }

    fn update_dir_x(&mut self, player1: &mut HumanPlayer, bot: &mut BotPlayer) {
        if self.collision_with_player(player1) || self.collision_with_bot(bot) {
            self.dir_x = match self.dir_x {
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
                _ => self.dir_x
            }
        }
    }

    fn collision_with_player(&self, player: &mut HumanPlayer) -> bool {
        return self.x <= RADIUS + human_player::X + player::WIDTH && self.x > human_player::X + RADIUS && 
            self.y >= player.player().y() && self.y <= player.player().y() + player::HEIGHT
    }

    fn collision_with_bot(&self, bot: &mut BotPlayer) -> bool {
        return self.x >= bot.x() - player::WIDTH - RADIUS && self.x < bot.x() - RADIUS &&
            self.y >= bot.player().y() && self.y <= bot.player().y() + player::HEIGHT
    }

    fn update_dir_y(&mut self, screen_height: f32) {
        if self.y <= RADIUS || self.y >= screen_height - RADIUS {
            self.dir_y = match self.dir_y {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                _ => self.dir_y
            }
        }
    }

    fn update_x(&mut self, delta_time: f32) {
        match self.dir_x {
            Direction::Left => self.x -= self.speed * delta_time,
            Direction::Right => self.x += self.speed * delta_time,
            _ => {}
        }
    }

    fn update_y(&mut self, delta_time: f32) {
        match self.dir_y {
            Direction::Up => self.y -= self.speed * delta_time,
            Direction::Down => self.y += self.speed * delta_time,
            _ => {}
        }
    }
}