use super::{level::Level, math::Vec2};

/// Mueve un circulo por pasos cortos y resuelve X/Y por separado. Esto evita
/// atravesar paredes con cuadros largos y permite deslizarse por sus caras.
pub fn move_with_collision(level: &Level, start: Vec2, translation: Vec2, radius: f32) -> Vec2 {
    if !translation.x.is_finite()
        || !translation.y.is_finite()
        || !radius.is_finite()
        || radius <= 0.0
    {
        return start;
    }

    let distance = translation.length_squared().sqrt();
    if distance <= f32::EPSILON {
        return start;
    }

    let maximum_step = (radius * 0.5).max(0.01);
    let step_count = (distance / maximum_step).ceil().max(1.0) as usize;
    let step = translation * (1.0 / step_count as f32);
    let mut position = start;

    for _ in 0..step_count {
        let candidate_x = Vec2::new(position.x + step.x, position.y);
        if !circle_intersects_solid(level, candidate_x, radius) {
            position.x = candidate_x.x;
        }

        let candidate_y = Vec2::new(position.x, position.y + step.y);
        if !circle_intersects_solid(level, candidate_y, radius) {
            position.y = candidate_y.y;
        }
    }

    position
}

pub fn circle_intersects_solid(level: &Level, center: Vec2, radius: f32) -> bool {
    if !center.x.is_finite() || !center.y.is_finite() || !radius.is_finite() || radius <= 0.0 {
        return true;
    }

    let minimum_column = (center.x - radius).floor() as i32;
    let maximum_column = (center.x + radius).floor() as i32;
    let minimum_row = (center.y - radius).floor() as i32;
    let maximum_row = (center.y + radius).floor() as i32;
    let radius_squared = radius * radius;

    for row in minimum_row..=maximum_row {
        for column in minimum_column..=maximum_column {
            if !level.is_solid(column, row) {
                continue;
            }

            let closest_x = center.x.clamp(column as f32, column as f32 + 1.0);
            let closest_y = center.y.clamp(row as f32, row as f32 + 1.0);
            let offset_x = center.x - closest_x;
            let offset_y = center.y - closest_y;

            if offset_x * offset_x + offset_y * offset_y < radius_squared {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{circle_intersects_solid, move_with_collision};
    use crate::game::{level::Level, math::Vec2};

    const OPEN_ROOM: &str = "\
11111
1S..1
1...1
1...1
11111";

    const ROOM_WITH_DIVIDER: &str = "\
111111
1S...1
1..1.1
1....1
111111";

    #[test]
    fn moves_freely_through_empty_space() {
        let level = Level::parse(OPEN_ROOM).expect("valid room");
        let result = move_with_collision(&level, Vec2::new(1.5, 1.5), Vec2::new(1.0, 1.0), 0.2);

        assert!((result.x - 2.5).abs() < 0.000_1);
        assert!((result.y - 2.5).abs() < 0.000_1);
    }

    #[test]
    fn stops_before_a_wall_even_with_a_large_translation() {
        let level = Level::parse(ROOM_WITH_DIVIDER).expect("valid room");
        let result = move_with_collision(&level, Vec2::new(1.5, 2.5), Vec2::new(8.0, 0.0), 0.22);

        assert!(result.x >= 1.5);
        assert!(result.x < 2.8);
        assert!(!circle_intersects_solid(&level, result, 0.22));
    }

    #[test]
    fn slides_along_a_wall_instead_of_stopping_completely() {
        let level = Level::parse(OPEN_ROOM).expect("valid room");
        let result = move_with_collision(&level, Vec2::new(1.25, 1.5), Vec2::new(-0.5, 1.0), 0.22);

        assert!(result.x >= 1.22);
        assert!(result.y > 2.3);
        assert!(!circle_intersects_solid(&level, result, 0.22));
    }

    #[test]
    fn treats_positions_outside_the_level_as_solid() {
        let level = Level::parse(OPEN_ROOM).expect("valid room");

        assert!(circle_intersects_solid(&level, Vec2::new(-0.1, 2.0), 0.22));
    }
}
