use super::math::Vec2;
use crate::config;

/// Camara del jugador en el plano 2D del nivel.
pub struct Player {
    pub position: Vec2,
    pub direction: Vec2,
    pub camera_plane: Vec2,
}

impl Player {
    pub fn at(position: Vec2) -> Self {
        Self {
            position,
            direction: Vec2::new(1.0, 0.0),
            // Un plano de 0.66 produce un campo de vision horizontal de ~67 grados.
            camera_plane: Vec2::new(0.0, config::CAMERA_PLANE_LENGTH),
        }
    }

    pub fn movement_direction(&self, forward_axis: f32, strafe_axis: f32) -> Vec2 {
        let right = Vec2::new(-self.direction.y, self.direction.x);
        (self.direction * forward_axis + right * strafe_axis).normalized_or_zero()
    }

    pub fn rotate(&mut self, angle_radians: f32) {
        if !angle_radians.is_finite() {
            return;
        }

        let (sine, cosine) = angle_radians.sin_cos();
        self.direction = Vec2::new(
            self.direction.x * cosine - self.direction.y * sine,
            self.direction.x * sine + self.direction.y * cosine,
        )
        .normalized_or_zero();
        self.camera_plane =
            Vec2::new(-self.direction.y, self.direction.x) * config::CAMERA_PLANE_LENGTH;
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::Player;
    use crate::{config, game::math::Vec2};

    #[test]
    fn diagonal_movement_is_normalized() {
        let player = Player::at(Vec2::new(2.5, 2.5));
        let movement = player.movement_direction(1.0, 1.0);

        assert!((movement.length_squared() - 1.0).abs() < 0.000_1);
        assert!(movement.x > 0.0);
        assert!(movement.y > 0.0);
    }

    #[test]
    fn opposing_or_empty_input_produces_no_movement() {
        let player = Player::at(Vec2::new(2.5, 2.5));

        assert_eq!(player.movement_direction(0.0, 0.0), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn rotation_keeps_camera_basis_perpendicular_and_normalized() {
        let mut player = Player::at(Vec2::new(2.5, 2.5));

        player.rotate(FRAC_PI_2);

        assert!(player.direction.x.abs() < 0.000_1);
        assert!((player.direction.y - 1.0).abs() < 0.000_1);
        assert!((player.direction.length_squared() - 1.0).abs() < 0.000_1);
        assert!((player.camera_plane.x + config::CAMERA_PLANE_LENGTH).abs() < 0.000_1);
        assert!(player.camera_plane.y.abs() < 0.000_1);
        let dot_product =
            player.direction.x * player.camera_plane.x + player.direction.y * player.camera_plane.y;
        assert!(dot_product.abs() < 0.000_1);
    }

    #[test]
    fn rotation_ignores_non_finite_angles() {
        let mut player = Player::at(Vec2::new(2.5, 2.5));
        let original_direction = player.direction;

        player.rotate(f32::NAN);

        assert_eq!(player.direction, original_direction);
    }
}
