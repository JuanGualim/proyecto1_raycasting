use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{
    config,
    game::{
        Game,
        entities::{Entity, EntityKind},
        player::Player,
    },
};

const SPRITE_WIDTH: i32 = 16;
const SPRITE_HEIGHT: i32 = 24;
const MINIMUM_DEPTH: f32 = 0.05;

#[derive(Debug, Clone, Copy)]
struct SpriteProjection {
    depth: f32,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
}

pub fn draw(drawing: &mut RaylibDrawHandle<'_>, game: &Game, depth_buffer: &[f32]) {
    let mut visible_entities: Vec<&Entity> = game
        .entities()
        .iter()
        .filter(|entity| entity.active)
        .collect();
    visible_entities.sort_by(|left, right| {
        distance_squared(right, game.player()).total_cmp(&distance_squared(left, game.player()))
    });

    for entity in visible_entities {
        let Some(projection) = project(entity, game.player()) else {
            continue;
        };
        draw_projected_sprite(drawing, entity, projection, depth_buffer);
    }
}

fn project(entity: &Entity, player: &Player) -> Option<SpriteProjection> {
    let relative_x = entity.position.x - player.position.x;
    let relative_y = entity.position.y - player.position.y;
    let determinant =
        player.camera_plane.x * player.direction.y - player.direction.x * player.camera_plane.y;

    if determinant.abs() <= f32::EPSILON {
        return None;
    }

    let inverse_determinant = determinant.recip();
    let camera_x =
        inverse_determinant * (player.direction.y * relative_x - player.direction.x * relative_y);
    let depth = inverse_determinant
        * (-player.camera_plane.y * relative_x + player.camera_plane.x * relative_y);

    if !depth.is_finite() || depth <= MINIMUM_DEPTH || !camera_x.is_finite() {
        return None;
    }

    let (height_scale, width_scale) = sprite_scale(entity.kind);
    let height = (config::WINDOW_HEIGHT as f32 / depth) * height_scale;
    let width = height * width_scale;
    let center_x = config::WINDOW_WIDTH as f32 * 0.5 * (1.0 + camera_x / depth);
    let center_y = config::WINDOW_HEIGHT as f32 * 0.5;

    if !height.is_finite() || !width.is_finite() || height <= 0.0 || width <= 0.0 {
        return None;
    }

    Some(SpriteProjection {
        depth,
        left: center_x - width * 0.5,
        top: center_y - height * 0.5,
        width,
        height,
    })
}

fn draw_projected_sprite(
    drawing: &mut RaylibDrawHandle<'_>,
    entity: &Entity,
    projection: SpriteProjection,
    depth_buffer: &[f32],
) {
    let first_column = projection.left.floor().max(0.0) as i32;
    let last_column = (projection.left + projection.width)
        .ceil()
        .min((config::WINDOW_WIDTH - 1) as f32) as i32;

    for screen_x in first_column..=last_column {
        let Some(&wall_depth) = depth_buffer.get(screen_x as usize) else {
            continue;
        };
        if !is_in_front_of_wall(projection.depth, wall_depth) {
            continue;
        }

        let texture_x = (((screen_x as f32 - projection.left) / projection.width)
            * SPRITE_WIDTH as f32)
            .floor()
            .clamp(0.0, (SPRITE_WIDTH - 1) as f32) as i32;

        for texture_y in 0..SPRITE_HEIGHT {
            let Some(color) = sample_sprite(entity, texture_x, texture_y) else {
                continue;
            };
            let block_top =
                projection.top + texture_y as f32 / SPRITE_HEIGHT as f32 * projection.height;
            let block_bottom =
                projection.top + (texture_y + 1) as f32 / SPRITE_HEIGHT as f32 * projection.height;
            let screen_top = block_top.floor().max(0.0) as i32;
            let screen_bottom = block_bottom.ceil().min((config::WINDOW_HEIGHT - 1) as f32) as i32;

            if screen_top <= screen_bottom {
                drawing.draw_line(screen_x, screen_top, screen_x, screen_bottom, color);
            }
        }
    }
}

fn sprite_scale(kind: EntityKind) -> (f32, f32) {
    match kind {
        EntityKind::Key => (0.55, 0.62),
        EntityKind::Portal => (1.15, 0.78),
        EntityKind::Guardian => (0.95, 0.68),
    }
}

fn sample_sprite(entity: &Entity, x: i32, y: i32) -> Option<Color> {
    match entity.kind {
        EntityKind::Key => sample_key(x, y),
        EntityKind::Portal => sample_portal(x, y),
        EntityKind::Guardian => sample_guardian(x, y, entity.hit_flash_remaining > 0.0),
    }
}

fn sample_key(x: i32, y: i32) -> Option<Color> {
    const GOLD: Color = Color::new(255, 205, 70, 255);
    const HIGHLIGHT: Color = Color::new(255, 244, 165, 255);
    let offset_x = x as f32 - 7.5;
    let offset_y = y as f32 - 6.5;
    let radius_squared = offset_x * offset_x + offset_y * offset_y;
    let ring = (7.0..=22.0).contains(&radius_squared) && y <= 11;
    let shaft = (7..=9).contains(&x) && (10..=21).contains(&y);
    let tooth = ((9..=13).contains(&x) && (16..=18).contains(&y))
        || ((9..=12).contains(&x) && (20..=22).contains(&y));

    if x == 7 && ring {
        Some(HIGHLIGHT)
    } else if ring || shaft || tooth {
        Some(GOLD)
    } else {
        None
    }
}

fn sample_portal(x: i32, y: i32) -> Option<Color> {
    const FRAME: Color = Color::new(83, 211, 204, 255);
    const FRAME_LIGHT: Color = Color::new(164, 255, 232, 255);
    const ENERGY: Color = Color::new(111, 61, 190, 235);
    const ENERGY_LIGHT: Color = Color::new(190, 97, 255, 240);
    let offset_x = x as f32 - 7.5;
    let offset_y = y as f32 - 7.5;
    let radius_squared = offset_x * offset_x + offset_y * offset_y;
    let arch = (22.0..=49.0).contains(&radius_squared) && y <= 9;
    let pillars = (1..=4).contains(&x) || (11..=14).contains(&x);
    let inner_energy = (5..=10).contains(&x) && (7..=22).contains(&y);

    if (arch || (pillars && (7..=23).contains(&y))) && (x == 3 || y <= 4) {
        Some(FRAME_LIGHT)
    } else if arch || (pillars && (7..=23).contains(&y)) {
        Some(FRAME)
    } else if inner_energy && (x + y) % 5 == 0 {
        Some(ENERGY_LIGHT)
    } else if inner_energy {
        Some(ENERGY)
    } else {
        None
    }
}

fn sample_guardian(x: i32, y: i32, hit_flash: bool) -> Option<Color> {
    const ARMOR: Color = Color::new(74, 53, 100, 255);
    const ARMOR_LIGHT: Color = Color::new(129, 92, 155, 255);
    const SHADOW: Color = Color::new(38, 28, 55, 255);
    const EYE: Color = Color::new(255, 66, 74, 255);
    const CORE: Color = Color::new(255, 142, 56, 255);

    let horns =
        (y <= 4 && ((2..=4).contains(&x) || (11..=13).contains(&x))) && (x + y <= 6 || x - y >= 9);
    let head = (4..=10).contains(&y) && (4..=11).contains(&x);
    let eyes = (y == 7 || y == 8) && (x == 5 || x == 10);
    let torso = (10..=19).contains(&y) && (3..=12).contains(&x);
    let arms = (11..=16).contains(&y) && (1..=14).contains(&x);
    let legs = (19..=23).contains(&y) && ((4..=6).contains(&x) || (9..=11).contains(&x));

    let base_color = if eyes {
        Some(EYE)
    } else if y == 14 && (7..=8).contains(&x) {
        Some(CORE)
    } else if horns || (head && (x == 4 || x == 11)) {
        Some(ARMOR_LIGHT)
    } else if head || torso || arms || legs {
        if (x + y) % 4 == 0 {
            Some(SHADOW)
        } else {
            Some(ARMOR)
        }
    } else {
        None
    };

    if hit_flash && base_color.is_some() {
        Some(Color::new(255, 216, 205, 255))
    } else {
        base_color
    }
}

fn distance_squared(entity: &Entity, player: &Player) -> f32 {
    let x = entity.position.x - player.position.x;
    let y = entity.position.y - player.position.y;
    x * x + y * y
}

fn is_in_front_of_wall(sprite_depth: f32, wall_depth: f32) -> bool {
    sprite_depth > MINIMUM_DEPTH && sprite_depth < wall_depth
}

#[cfg(test)]
mod tests {
    use super::{is_in_front_of_wall, project};
    use crate::game::{
        entities::{Entity, EntityKind},
        math::Vec2,
        player::Player,
    };

    fn guardian_at(position: Vec2) -> Entity {
        Entity {
            kind: EntityKind::Guardian,
            position,
            active: true,
            health: crate::config::GUARDIAN_MAX_HEALTH,
            hit_flash_remaining: 0.0,
        }
    }

    #[test]
    fn entity_in_front_projects_to_screen_center() {
        let player = Player::at(Vec2::new(2.5, 2.5));
        let projection =
            project(&guardian_at(Vec2::new(4.5, 2.5)), &player).expect("entity should be visible");
        let center = projection.left + projection.width * 0.5;

        assert!((projection.depth - 2.0).abs() < 0.000_1);
        assert!((center - 480.0).abs() < 0.000_1);
    }

    #[test]
    fn entity_behind_player_is_not_projected() {
        let player = Player::at(Vec2::new(2.5, 2.5));

        assert!(project(&guardian_at(Vec2::new(1.5, 2.5)), &player).is_none());
    }

    #[test]
    fn wall_depth_occludes_sprite_stripes() {
        assert!(is_in_front_of_wall(2.0, 3.0));
        assert!(!is_in_front_of_wall(3.0, 2.0));
        assert!(!is_in_front_of_wall(2.0, 2.0));
    }
}
