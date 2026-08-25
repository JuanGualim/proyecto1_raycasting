use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{
    config,
    game::{
        Game,
        level::Material,
        raycast::{HitSide, cast_camera_ray},
    },
};

const CEILING_TOP: (u8, u8, u8) = (10, 9, 25);
const CEILING_HORIZON: (u8, u8, u8) = (48, 34, 61);
const FLOOR_HORIZON: (u8, u8, u8) = (48, 35, 31);
const FLOOR_BOTTOM: (u8, u8, u8) = (13, 12, 17);

pub fn draw(drawing: &mut RaylibDrawHandle<'_>, game: &Game) {
    draw_background(drawing);

    for screen_x in 0..config::WINDOW_WIDTH {
        let camera_x = 2.0 * screen_x as f32 / config::WINDOW_WIDTH as f32 - 1.0;
        let Some(hit) = cast_camera_ray(game.level(), game.player(), camera_x) else {
            continue;
        };

        let line_height = (config::WINDOW_HEIGHT as f32 / hit.distance)
            .round()
            .clamp(1.0, (config::WINDOW_HEIGHT * 8) as f32) as i32;
        let line_start = ((config::WINDOW_HEIGHT - line_height) / 2).max(0);
        let line_end = ((config::WINDOW_HEIGHT + line_height) / 2).min(config::WINDOW_HEIGHT - 1);

        drawing.draw_line(
            screen_x,
            line_start,
            screen_x,
            line_end,
            shaded_wall_color(hit.material, hit.side, hit.distance),
        );
    }
}

fn draw_background(drawing: &mut RaylibDrawHandle<'_>) {
    let horizon = config::WINDOW_HEIGHT / 2;

    for y in 0..horizon {
        let amount = y as f32 / horizon as f32;
        drawing.draw_line(
            0,
            y,
            config::WINDOW_WIDTH,
            y,
            interpolate_color(CEILING_TOP, CEILING_HORIZON, amount),
        );
    }

    for y in horizon..config::WINDOW_HEIGHT {
        let amount = (y - horizon) as f32 / horizon as f32;
        drawing.draw_line(
            0,
            y,
            config::WINDOW_WIDTH,
            y,
            interpolate_color(FLOOR_HORIZON, FLOOR_BOTTOM, amount),
        );
    }
}

fn shaded_wall_color(material: Material, side: HitSide, distance: f32) -> Color {
    let base = match material {
        Material::Stone => (102, 126, 151),
        Material::Obsidian => (121, 72, 181),
        Material::Brick => (190, 69, 67),
        Material::Glyph => (224, 169, 55),
        Material::Moss => (63, 153, 93),
    };
    let side_shade = match side {
        HitSide::Vertical => 1.0,
        HitSide::Horizontal => 0.72,
    };
    let distance_shade = (1.0 / (1.0 + distance * 0.075)).clamp(0.38, 1.0);

    scale_color(base, side_shade * distance_shade)
}

fn scale_color(color: (u8, u8, u8), scale: f32) -> Color {
    Color::new(
        (color.0 as f32 * scale) as u8,
        (color.1 as f32 * scale) as u8,
        (color.2 as f32 * scale) as u8,
        255,
    )
}

fn interpolate_color(from: (u8, u8, u8), to: (u8, u8, u8), amount: f32) -> Color {
    let mix = |start: u8, end: u8| {
        (start as f32 + (end as f32 - start as f32) * amount.clamp(0.0, 1.0)) as u8
    };
    Color::new(mix(from.0, to.0), mix(from.1, to.1), mix(from.2, to.2), 255)
}
