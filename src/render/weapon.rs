use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{
    config,
    game::{Game, combat::ShotOutcome},
};

const METAL_DARK: Color = Color::new(31, 27, 42, 255);
const METAL: Color = Color::new(69, 60, 82, 255);
const METAL_LIGHT: Color = Color::new(122, 104, 132, 255);
const ENERGY: Color = Color::new(239, 184, 72, 255);

pub fn draw(drawing: &mut RaylibDrawHandle<'_>, game: &Game) {
    let center_x = config::WINDOW_WIDTH / 2;
    let bottom = config::WINDOW_HEIGHT;

    drawing.draw_rectangle(center_x - 58, bottom - 54, 116, 54, METAL_DARK);
    drawing.draw_rectangle(center_x - 39, bottom - 86, 78, 86, METAL);
    drawing.draw_rectangle(center_x - 22, bottom - 120, 44, 72, METAL_DARK);
    drawing.draw_rectangle(center_x - 13, bottom - 132, 26, 56, METAL_LIGHT);
    drawing.draw_rectangle(center_x - 8, bottom - 130, 7, 54, ENERGY);

    let flash_strength = game.muzzle_flash_strength();
    if flash_strength > 0.0 {
        let muzzle_y = bottom - 139;
        let outer_radius = 24.0 * flash_strength.max(0.35);
        drawing.draw_circle(
            center_x,
            muzzle_y,
            outer_radius,
            Color::new(255, 111, 41, 150),
        );
        drawing.draw_circle(
            center_x,
            muzzle_y,
            outer_radius * 0.58,
            Color::new(255, 221, 92, 235),
        );
        drawing.draw_line(
            center_x,
            muzzle_y - outer_radius as i32 - 8,
            center_x,
            muzzle_y + 8,
            Color::new(255, 245, 190, 220),
        );
        drawing.draw_line(
            center_x - outer_radius as i32 - 8,
            muzzle_y,
            center_x + outer_radius as i32 + 8,
            muzzle_y,
            Color::new(255, 205, 76, 210),
        );
    }

    if let Some(outcome) = game.visible_shot_feedback() {
        draw_feedback(drawing, outcome);
    }
}

fn draw_feedback(drawing: &mut RaylibDrawHandle<'_>, outcome: ShotOutcome) {
    let (label, color) = match outcome {
        ShotOutcome::Hit { destroyed: true } => {
            ("GUARDIAN ELIMINADO", Color::new(255, 210, 86, 255))
        }
        ShotOutcome::Hit { destroyed: false } => ("IMPACTO", Color::new(255, 105, 91, 255)),
        ShotOutcome::Blocked => ("BLOQUEADO POR MURO", Color::new(130, 190, 255, 255)),
        ShotOutcome::Miss => ("FALLO", Color::new(205, 198, 215, 255)),
        ShotOutcome::Cooldown => return,
    };
    let font_size = 18;
    let width = drawing.measure_text(label, font_size);
    drawing.draw_text(
        label,
        (config::WINDOW_WIDTH - width) / 2,
        config::WINDOW_HEIGHT / 2 + 24,
        font_size,
        color,
    );
}
