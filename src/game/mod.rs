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

    pub fn rotate_player(&mut self, angle_radians: f32) {
        self.player.rotate(angle_radians);
    }

    pub fn reset_player(&mut self) {
        self.player = Player::at(self.level.spawn());
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::{
        Game,
        collision::circle_intersects_solid,
        level::Material,
        raycast::{HitSide, cast_camera_ray},
    };

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

    #[test]
    fn rotation_changes_the_center_ray_without_distorting_distance() {
        let mut game = Game::load_first_level().expect("embedded level should be valid");

        game.rotate_player(FRAC_PI_2);
        let hit = cast_camera_ray(game.level(), game.player(), 0.0).expect("south wall hit");

        assert_eq!(hit.material, Material::Glyph);
        assert_eq!(hit.side, HitSide::Horizontal);
        assert!((hit.distance - 2.5).abs() < 0.000_1);
    }

    #[test]
    fn repeated_navigation_never_places_player_inside_a_wall() {
        let mut game = Game::load_first_level().expect("embedded level should be valid");

        for step in 0..720 {
            if step % 90 == 0 {
                game.rotate_player(FRAC_PI_2);
            }
            game.move_player(1.0, 0.35, 1.0 / 60.0);

            assert!(!circle_intersects_solid(
                game.level(),
                game.player().position,
                crate::config::PLAYER_RADIUS,
            ));
        }
    }

    #[test]
    fn every_screen_column_hits_a_finite_wall_from_spawn() {
        let game = Game::load_first_level().expect("embedded level should be valid");

        for column in 0..crate::config::WINDOW_WIDTH {
            let camera_x = 2.0 * column as f32 / crate::config::WINDOW_WIDTH as f32 - 1.0;
            let hit = cast_camera_ray(game.level(), game.player(), camera_x)
                .expect("closed level should stop every ray");

            assert!(hit.distance.is_finite());
            assert!(hit.distance > 0.0);
        }
    }
}
