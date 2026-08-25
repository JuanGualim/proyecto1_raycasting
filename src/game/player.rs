use super::math::Vec2;

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
            camera_plane: Vec2::new(0.0, 0.66),
        }
    }

    pub fn movement_direction(&self, forward_axis: f32, strafe_axis: f32) -> Vec2 {
        let right = Vec2::new(-self.direction.y, self.direction.x);
        (self.direction * forward_axis + right * strafe_axis).normalized_or_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::Player;
    use crate::game::math::Vec2;

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
}
