pub mod collision;
pub mod combat;
pub mod entities;
pub mod level;
pub mod math;
pub mod player;
pub mod raycast;

use combat::{ShotOutcome, ray_circle_hit_distance};
use entities::{Entity, EntityKind};
use level::{Level, LevelError};
use player::Player;
use raycast::cast_ray;

const ECLIPSE_CHAMBER_ONE: &str = include_str!("../../levels/eclipse_1.txt");

pub struct Game {
    level: Level,
    player: Player,
    entities: Vec<Entity>,
    shot_cooldown_remaining: f32,
    muzzle_flash_remaining: f32,
    shot_feedback_remaining: f32,
    last_shot: Option<ShotOutcome>,
}

impl Game {
    pub fn load_first_level() -> Result<Self, LevelError> {
        let level = Level::parse(ECLIPSE_CHAMBER_ONE)?;
        Ok(Self::from_level(level))
    }

    fn from_level(level: Level) -> Self {
        let player = Player::at(level.spawn());
        let entities = level
            .entity_spawns()
            .iter()
            .copied()
            .map(Entity::from)
            .collect();

        Self {
            level,
            player,
            entities,
            shot_cooldown_remaining: 0.0,
            muzzle_flash_remaining: 0.0,
            shot_feedback_remaining: 0.0,
            last_shot: None,
        }
    }

    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
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

    pub fn tick(&mut self, delta_time: f32) {
        if !delta_time.is_finite() || delta_time <= 0.0 {
            return;
        }

        self.shot_cooldown_remaining = (self.shot_cooldown_remaining - delta_time).max(0.0);
        self.muzzle_flash_remaining = (self.muzzle_flash_remaining - delta_time).max(0.0);
        self.shot_feedback_remaining = (self.shot_feedback_remaining - delta_time).max(0.0);
        for entity in &mut self.entities {
            entity.hit_flash_remaining = (entity.hit_flash_remaining - delta_time).max(0.0);
        }
    }

    pub fn try_shoot(&mut self) -> ShotOutcome {
        if self.shot_cooldown_remaining > 0.0 {
            return ShotOutcome::Cooldown;
        }

        self.shot_cooldown_remaining = crate::config::SHOT_COOLDOWN;
        self.muzzle_flash_remaining = crate::config::MUZZLE_FLASH_DURATION;

        let wall_distance = cast_ray(&self.level, self.player.position, self.player.direction)
            .map_or(f32::INFINITY, |hit| hit.distance);
        let nearest_guardian = self
            .entities
            .iter()
            .enumerate()
            .filter(|(_, entity)| entity.active && entity.kind == EntityKind::Guardian)
            .filter_map(|(index, entity)| {
                ray_circle_hit_distance(
                    self.player.position,
                    self.player.direction,
                    entity.position,
                    crate::config::GUARDIAN_HIT_RADIUS,
                )
                .map(|distance| (index, distance))
            })
            .min_by(|left, right| left.1.total_cmp(&right.1));

        let outcome = match nearest_guardian {
            None => ShotOutcome::Miss,
            Some((_, distance)) if distance >= wall_distance => ShotOutcome::Blocked,
            Some((index, _)) => {
                let guardian = &mut self.entities[index];
                guardian.health = (guardian.health - 1).max(0);
                guardian.hit_flash_remaining = crate::config::GUARDIAN_HIT_FLASH_DURATION;
                let destroyed = guardian.health == 0;
                if destroyed {
                    guardian.active = false;
                }
                ShotOutcome::Hit { destroyed }
            }
        };

        self.last_shot = Some(outcome);
        self.shot_feedback_remaining = crate::config::SHOT_FEEDBACK_DURATION;
        outcome
    }

    pub fn can_shoot(&self) -> bool {
        self.shot_cooldown_remaining <= 0.0
    }

    pub fn muzzle_flash_strength(&self) -> f32 {
        (self.muzzle_flash_remaining / crate::config::MUZZLE_FLASH_DURATION).clamp(0.0, 1.0)
    }

    pub fn visible_shot_feedback(&self) -> Option<ShotOutcome> {
        (self.shot_feedback_remaining > 0.0)
            .then_some(self.last_shot)
            .flatten()
    }

    pub fn guardian_health(&self) -> i32 {
        self.entities
            .iter()
            .find(|entity| entity.kind == EntityKind::Guardian)
            .map_or(0, |guardian| guardian.health)
    }

    pub fn reset_level(&mut self) {
        self.player = Player::at(self.level.spawn());
        self.entities = self
            .level
            .entity_spawns()
            .iter()
            .copied()
            .map(Entity::from)
            .collect();
        self.shot_cooldown_remaining = 0.0;
        self.muzzle_flash_remaining = 0.0;
        self.shot_feedback_remaining = 0.0;
        self.last_shot = None;
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::{
        Game,
        collision::circle_intersects_solid,
        combat::ShotOutcome,
        entities::EntityKind,
        level::{Level, Material},
        raycast::{HitSide, cast_camera_ray},
    };

    #[test]
    fn movement_uses_speed_and_delta_time_then_can_be_reset() {
        let mut game = Game::load_first_level().expect("embedded level should be valid");
        let spawn = game.player().position;

        game.move_player(1.0, 0.0, 0.1);

        assert!((game.player().position.x - (spawn.x + 0.3)).abs() < 0.000_1);
        assert!((game.player().position.y - spawn.y).abs() < 0.000_1);

        game.reset_level();
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

    #[test]
    fn embedded_level_instantiates_key_portal_and_guardian() {
        let game = Game::load_first_level().expect("embedded level should be valid");

        assert_eq!(game.entities().len(), 3);
        assert!(
            [EntityKind::Key, EntityKind::Portal, EntityKind::Guardian]
                .into_iter()
                .all(|kind| game
                    .entities()
                    .iter()
                    .any(|entity| entity.kind == kind && entity.active))
        );
    }

    #[test]
    fn shots_damage_destroy_and_reset_the_guardian() {
        let mut game = Game::load_first_level().expect("embedded level should be valid");

        assert_eq!(game.try_shoot(), ShotOutcome::Hit { destroyed: false });
        assert_eq!(game.guardian_health(), 2);
        assert!(!game.can_shoot());
        assert_eq!(game.try_shoot(), ShotOutcome::Cooldown);
        assert!(game.muzzle_flash_strength() > 0.0);

        game.tick(crate::config::SHOT_COOLDOWN);
        assert_eq!(game.try_shoot(), ShotOutcome::Hit { destroyed: false });
        game.tick(crate::config::SHOT_COOLDOWN);
        assert_eq!(game.try_shoot(), ShotOutcome::Hit { destroyed: true });
        assert_eq!(game.guardian_health(), 0);
        assert!(
            game.entities()
                .iter()
                .any(|entity| entity.kind == EntityKind::Guardian && !entity.active)
        );

        game.reset_level();
        assert_eq!(game.guardian_health(), crate::config::GUARDIAN_MAX_HEALTH);
        assert!(game.can_shoot());
    }

    #[test]
    fn wall_blocks_a_guardian_aligned_with_the_crosshair() {
        let level = Level::parse("1111111\n1S.1G.1\n1.....1\n1111111").expect("valid room");
        let mut game = Game::from_level(level);

        assert_eq!(game.try_shoot(), ShotOutcome::Blocked);
        assert_eq!(game.guardian_health(), crate::config::GUARDIAN_MAX_HEALTH);
    }

    #[test]
    fn shot_misses_when_no_guardian_intersects_the_center_ray() {
        let level = Level::parse("11111\n1S..1\n1.G.1\n1...1\n11111").expect("valid room");
        let mut game = Game::from_level(level);

        assert_eq!(game.try_shoot(), ShotOutcome::Miss);
        assert_eq!(game.guardian_health(), crate::config::GUARDIAN_MAX_HEALTH);
    }
}
