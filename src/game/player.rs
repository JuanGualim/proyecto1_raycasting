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
}
