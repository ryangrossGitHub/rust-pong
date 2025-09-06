pub const HEIGHT: f32 = 100.0;
pub const WIDTH: f32 = 10.0;

pub struct Player {
    y: f32,
    score: u32,
}

impl Player {
    pub fn new(y: f32) -> Player {
        Player { y, score: 0 }
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn add_y(&mut self, y: f32) {
        self.y += y;
    }

    pub fn subtract_y(&mut self, y: f32) {
        self.y -= y;
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}