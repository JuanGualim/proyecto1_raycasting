pub mod level;
pub mod math;
pub mod player;
pub mod raycast;

use level::{Level, LevelError};
use player::Player;

const ECLIPSE_CHAMBER_ONE: &str = include_str!("../../levels/eclipse_1.txt");

pub struct Game {
    level: Level,
    player: Player,
}

impl Game {
    pub fn load_first_level() -> Result<Self, LevelError> {
        let level = Level::parse(ECLIPSE_CHAMBER_ONE)?;
        let player = Player::at(level.spawn());

        Ok(Self { level, player })
    }

    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
}
