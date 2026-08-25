pub mod collision;
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

    pub fn move_player(&mut self, forward_axis: f32, strafe_axis: f32, delta_time: f32) {
        let direction = self.player.movement_direction(forward_axis, strafe_axis);
        let translation = direction * crate::config::PLAYER_MOVE_SPEED * delta_time;
        self.player.position = collision::move_with_collision(
            &self.level,
            self.player.position,
            translation,
            crate::config::PLAYER_RADIUS,
        );
    }

    pub fn reset_player(&mut self) {
        self.player = Player::at(self.level.spawn());
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn movement_uses_speed_and_delta_time_then_can_be_reset() {
        let mut game = Game::load_first_level().expect("embedded level should be valid");
        let spawn = game.player().position;

        game.move_player(1.0, 0.0, 0.1);

        assert!((game.player().position.x - (spawn.x + 0.3)).abs() < 0.000_1);
        assert!((game.player().position.y - spawn.y).abs() < 0.000_1);

        game.reset_player();
        assert_eq!(game.player().position, spawn);
    }
}
