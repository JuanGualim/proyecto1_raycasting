use super::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShotOutcome {
    Hit { destroyed: bool },
    Blocked,
    Miss,
    Cooldown,
}

/// Devuelve la primera distancia positiva en la que un rayo toca un circulo.
/// La direccion se normaliza para que la distancia sea comparable con DDA.
pub fn ray_circle_hit_distance(
    origin: Vec2,
    direction: Vec2,
    center: Vec2,
    radius: f32,
) -> Option<f32> {
    if !radius.is_finite() || radius <= 0.0 {
        return None;
    }

    let direction = direction.normalized_or_zero();
    if direction.length_squared() <= f32::EPSILON {
        return None;
    }

    let to_center = Vec2::new(center.x - origin.x, center.y - origin.y);
    let projected_distance = to_center.x * direction.x + to_center.y * direction.y;
    let perpendicular_squared =
        (to_center.length_squared() - projected_distance * projected_distance).max(0.0);
    let radius_squared = radius * radius;

    if perpendicular_squared > radius_squared {
        return None;
    }

    let half_chord = (radius_squared - perpendicular_squared).sqrt();
    let near = projected_distance - half_chord;
    let far = projected_distance + half_chord;

    if near >= 0.0 {
        Some(near)
    } else if far >= 0.0 {
        Some(far)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::ray_circle_hit_distance;
    use crate::game::math::Vec2;

    #[test]
    fn centered_target_is_hit_at_its_near_edge() {
        let distance = ray_circle_hit_distance(
            Vec2::new(1.0, 2.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(4.0, 2.0),
            0.5,
        )
        .expect("target should be hit");

        assert!((distance - 2.5).abs() < 0.000_1);
    }

    #[test]
    fn misses_offset_and_behind_targets() {
        assert!(
            ray_circle_hit_distance(
                Vec2::new(1.0, 2.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(4.0, 3.0),
                0.25,
            )
            .is_none()
        );
        assert!(
            ray_circle_hit_distance(
                Vec2::new(1.0, 2.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, 2.0),
                0.25,
            )
            .is_none()
        );
    }

    #[test]
    fn rejects_zero_direction_and_invalid_radius() {
        let origin = Vec2::new(1.0, 1.0);
        let target = Vec2::new(2.0, 1.0);

        assert!(ray_circle_hit_distance(origin, Vec2::new(0.0, 0.0), target, 0.2).is_none());
        assert!(ray_circle_hit_distance(origin, Vec2::new(1.0, 0.0), target, 0.0).is_none());
    }
}
