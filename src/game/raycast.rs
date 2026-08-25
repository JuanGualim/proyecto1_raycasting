use super::{
    level::{Level, Material},
    math::Vec2,
    player::Player,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitSide {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayHit {
    pub distance: f32,
    pub material: Material,
    pub side: HitSide,
}

pub fn cast_camera_ray(level: &Level, player: &Player, camera_x: f32) -> Option<RayHit> {
    let ray_direction = Vec2::new(
        player.direction.x + player.camera_plane.x * camera_x,
        player.direction.y + player.camera_plane.y * camera_x,
    );

    cast_ray(level, player.position, ray_direction)
}

pub fn cast_ray(level: &Level, origin: Vec2, direction: Vec2) -> Option<RayHit> {
    if direction.x.abs() < f32::EPSILON && direction.y.abs() < f32::EPSILON {
        return None;
    }

    let mut map_x = origin.x.floor() as i32;
    let mut map_y = origin.y.floor() as i32;

    if !level.contains(map_x, map_y) {
        return None;
    }

    let delta_distance_x = reciprocal_magnitude(direction.x);
    let delta_distance_y = reciprocal_magnitude(direction.y);

    let (step_x, mut side_distance_x) = if direction.x < 0.0 {
        (-1, (origin.x - map_x as f32) * delta_distance_x)
    } else {
        (1, (map_x as f32 + 1.0 - origin.x) * delta_distance_x)
    };
    let (step_y, mut side_distance_y) = if direction.y < 0.0 {
        (-1, (origin.y - map_y as f32) * delta_distance_y)
    } else {
        (1, (map_y as f32 + 1.0 - origin.y) * delta_distance_y)
    };

    let maximum_steps = level
        .width()
        .saturating_mul(level.height())
        .saturating_add(level.width())
        .saturating_add(level.height());

    for _ in 0..maximum_steps {
        let (side, distance) = if side_distance_x < side_distance_y {
            map_x += step_x;
            let distance = side_distance_x;
            side_distance_x += delta_distance_x;
            (HitSide::Vertical, distance)
        } else {
            map_y += step_y;
            let distance = side_distance_y;
            side_distance_y += delta_distance_y;
            (HitSide::Horizontal, distance)
        };

        if !level.contains(map_x, map_y) {
            return None;
        }

        if let Some(material) = level.wall_material_at(map_x, map_y) {
            if distance.is_finite() && distance >= 0.0 {
                return Some(RayHit {
                    distance: distance.max(0.000_1),
                    material,
                    side,
                });
            }
            return None;
        }
    }

    None
}

fn reciprocal_magnitude(component: f32) -> f32 {
    if component.abs() < f32::EPSILON {
        f32::INFINITY
    } else {
        (1.0 / component).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::{HitSide, cast_camera_ray, cast_ray};
    use crate::game::{level::Level, math::Vec2, player::Player};

    const TEST_ROOM: &str = "\
11111
1...2
1.S.2
1...2
11111";

    #[test]
    fn center_camera_ray_returns_perpendicular_distance() {
        let level = Level::parse(TEST_ROOM).expect("valid room");
        let player = Player::at(level.spawn());
        let hit = cast_camera_ray(&level, &player, 0.0).expect("east wall hit");

        assert!((hit.distance - 1.5).abs() < 0.000_1);
        assert_eq!(hit.side, HitSide::Vertical);
    }

    #[test]
    fn axis_aligned_rays_are_finite() {
        let level = Level::parse(TEST_ROOM).expect("valid room");
        let origin = level.spawn();
        let east = cast_ray(&level, origin, Vec2::new(1.0, 0.0)).expect("east hit");
        let north = cast_ray(&level, origin, Vec2::new(0.0, -1.0)).expect("north hit");

        assert!(east.distance.is_finite());
        assert!(north.distance.is_finite());
        assert_eq!(north.side, HitSide::Horizontal);
    }

    #[test]
    fn rejects_zero_length_ray_and_origin_outside_map() {
        let level = Level::parse(TEST_ROOM).expect("valid room");

        assert!(cast_ray(&level, level.spawn(), Vec2::new(0.0, 0.0)).is_none());
        assert!(cast_ray(&level, Vec2::new(-1.0, 2.0), Vec2::new(1.0, 0.0)).is_none());
    }
}
