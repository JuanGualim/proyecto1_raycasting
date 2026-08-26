use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{
    app::Screen,
    config,
    game::{Game, level::Material},
    render,
};

const NIGHT: Color = Color::new(11, 10, 24, 255);
const GOLD: Color = Color::new(239, 184, 72, 255);
const PALE_GOLD: Color = Color::new(255, 226, 156, 255);
const MUTED: Color = Color::new(171, 164, 188, 255);

pub fn draw(
    drawing: &mut RaylibDrawHandle<'_>,
    screen: Screen,
    selected_level: usize,
    elapsed_seconds: f32,
    game: &Game,
    level_load_error: Option<&str>,
) {
    drawing.clear_background(NIGHT);

    match screen {
        Screen::Welcome => draw_welcome(drawing, elapsed_seconds),
        Screen::LevelSelect => draw_level_select(drawing, selected_level, level_load_error),
        Screen::Playing => draw_game(drawing, game),
        Screen::Paused => {
            draw_game(drawing, game);
            draw_pause(drawing);
        }
        Screen::Victory => draw_victory(drawing, game),
    }
}

fn draw_welcome(drawing: &mut RaylibDrawHandle<'_>, elapsed_seconds: f32) {
    draw_centered(drawing, "TEMPLO DEL ECLIPSE", 160, 44, GOLD);
    draw_centered(drawing, "Un ray caster escrito en Rust", 224, 22, PALE_GOLD);

    if (elapsed_seconds * 2.0) as i32 % 2 == 0 {
        draw_centered(drawing, "ENTER  -  comenzar", 340, 22, Color::WHITE);
    }

    draw_centered(drawing, "Q  -  salir", 390, 18, MUTED);
    draw_phase_badge(drawing);
}

fn draw_level_select(
    drawing: &mut RaylibDrawHandle<'_>,
    selected_level: usize,
    level_load_error: Option<&str>,
) {
    draw_centered(drawing, "SELECCION DE NIVEL", 100, 34, GOLD);
    draw_centered(
        drawing,
        &format!("<   CAMARA {}   >", selected_level + 1),
        235,
        32,
        Color::WHITE,
    );
    draw_centered(
        drawing,
        &format!(
            "{} {} disponibles  |  ENTER para entrar",
            crate::game::catalog::level_count(),
            if crate::game::catalog::level_count() == 1 {
                "nivel"
            } else {
                "niveles"
            }
        ),
        325,
        20,
        MUTED,
    );
    draw_centered(drawing, "Q  -  salir", 380, 18, MUTED);
    if let Some(error) = level_load_error {
        draw_centered(drawing, error, 425, 16, Color::new(255, 120, 105, 255));
    }
    draw_phase_badge(drawing);
}

fn draw_game(drawing: &mut RaylibDrawHandle<'_>, game: &Game) {
    let depth_buffer = render::world::draw(drawing, game);
    render::sprites::draw(drawing, game, &depth_buffer);
    render::weapon::draw(drawing, game);

    drawing.draw_rectangle(12, 12, 445, 101, Color::new(8, 7, 18, 220));
    drawing.draw_text(
        &format!("CAMARA {}  |  RAY CASTING DDA", game.level_index() + 1),
        24,
        23,
        18,
        PALE_GOLD,
    );
    drawing.draw_text(
        "WASD mover  |  Mouse girar  |  Clic disparar",
        24,
        47,
        15,
        MUTED,
    );
    drawing.draw_text(
        &format!(
            "GUARDIAN {}/{}  |  DISPARO {}",
            game.guardian_health(),
            config::GUARDIAN_MAX_HEALTH,
            if game.can_shoot() {
                "LISTO"
            } else {
                "CARGANDO"
            }
        ),
        24,
        66,
        15,
        if game.guardian_health() > 0 {
            Color::new(255, 155, 112, 255)
        } else {
            Color::new(126, 224, 157, 255)
        },
    );
    drawing.draw_text(
        &format!(
            "LLAVE {}  |  PORTAL {}",
            if game.has_key() {
                "OBTENIDA"
            } else {
                "PENDIENTE"
            },
            if game.portal_ready() {
                "ACTIVO"
            } else {
                "BLOQUEADO"
            }
        ),
        24,
        86,
        15,
        if game.portal_ready() {
            Color::new(91, 239, 207, 255)
        } else {
            GOLD
        },
    );

    draw_material_legend(drawing);
    draw_crosshair(drawing, game);
    draw_interaction_feedback(drawing, game);
    render::minimap::draw(drawing, game);
}

fn draw_pause(drawing: &mut RaylibDrawHandle<'_>) {
    drawing.draw_rectangle(
        0,
        0,
        config::WINDOW_WIDTH,
        config::WINDOW_HEIGHT,
        Color::new(5, 4, 12, 210),
    );
    draw_centered(drawing, "PAUSA", 180, 46, GOLD);
    draw_centered(drawing, "ESC  -  continuar", 280, 22, Color::WHITE);
    draw_centered(drawing, "M  -  volver al selector", 320, 20, MUTED);
}

fn draw_victory(drawing: &mut RaylibDrawHandle<'_>, game: &Game) {
    for radius in (80..=360).step_by(40) {
        drawing.draw_circle_lines(
            config::WINDOW_WIDTH / 2,
            210,
            radius as f32,
            Color::new(68, 185, 169, 70),
        );
    }
    draw_centered(drawing, "TEMPLO PURIFICADO", 115, 46, GOLD);
    draw_centered(
        drawing,
        "Llave recuperada  |  Guardian neutralizado  |  Portal activado",
        198,
        19,
        PALE_GOLD,
    );
    draw_centered(
        drawing,
        &format!("TIEMPO  {:.1} segundos", game.level_elapsed_seconds()),
        250,
        25,
        Color::new(115, 238, 205, 255),
    );
    draw_centered(
        drawing,
        "ENTER  -  volver al selector",
        345,
        21,
        Color::WHITE,
    );
}

fn draw_phase_badge(drawing: &mut RaylibDrawHandle<'_>) {
    let label = "CICLO JUGABLE COMPLETO - FASE 3";
    let font_size = 16;
    let width = drawing.measure_text(label, font_size);
    drawing.draw_text(
        label,
        config::WINDOW_WIDTH - width - 18,
        config::WINDOW_HEIGHT - 30,
        font_size,
        MUTED,
    );
}

fn draw_material_legend(drawing: &mut RaylibDrawHandle<'_>) {
    const MATERIALS: [(&str, Material); 5] = [
        ("1 Piedra", Material::Stone),
        ("2 Obsidiana", Material::Obsidian),
        ("3 Ladrillo", Material::Brick),
        ("4 Glifos", Material::Glyph),
        ("5 Musgo", Material::Moss),
    ];

    let panel_x = 12;
    let panel_y = config::WINDOW_HEIGHT - 132;
    drawing.draw_rectangle(panel_x, panel_y, 140, 120, Color::new(8, 7, 18, 210));

    for (index, (label, material)) in MATERIALS.iter().enumerate() {
        let y = panel_y + 10 + index as i32 * 21;
        drawing.draw_rectangle(
            panel_x + 10,
            y,
            12,
            12,
            render::palette::material_color(*material),
        );
        drawing.draw_text(label, panel_x + 29, y - 1, 14, Color::WHITE);
    }
}

fn draw_crosshair(drawing: &mut RaylibDrawHandle<'_>, game: &Game) {
    let center_x = config::WINDOW_WIDTH / 2;
    let center_y = config::WINDOW_HEIGHT / 2;
    let color = match game.visible_shot_feedback() {
        Some(crate::game::combat::ShotOutcome::Hit { .. }) => Color::new(255, 92, 82, 255),
        Some(crate::game::combat::ShotOutcome::Blocked) => Color::new(116, 184, 255, 255),
        _ => Color::new(255, 238, 190, 220),
    };

    drawing.draw_line(center_x - 10, center_y, center_x - 4, center_y, color);
    drawing.draw_line(center_x + 4, center_y, center_x + 10, center_y, color);
    drawing.draw_line(center_x, center_y - 10, center_x, center_y - 4, color);
    drawing.draw_line(center_x, center_y + 4, center_x, center_y + 10, color);
    drawing.draw_circle(center_x, center_y, 1.5, GOLD);
}

fn draw_interaction_feedback(drawing: &mut RaylibDrawHandle<'_>, game: &Game) {
    use crate::game::objective::InteractionFeedback;

    let Some(feedback) = game.visible_interaction_feedback() else {
        return;
    };
    let (label, color) = match feedback {
        InteractionFeedback::KeyCollected => {
            ("LLAVE SOLAR OBTENIDA", Color::new(255, 218, 88, 255))
        }
        InteractionFeedback::PortalNeedsKey => (
            "EL PORTAL REQUIERE LA LLAVE",
            Color::new(185, 177, 203, 255),
        ),
        InteractionFeedback::PortalNeedsGuardian => (
            "EL GUARDIAN AUN PROTEGE EL PORTAL",
            Color::new(255, 125, 105, 255),
        ),
    };
    let font_size = 18;
    let width = drawing.measure_text(label, font_size);
    let x = (config::WINDOW_WIDTH - width) / 2;
    let y = config::WINDOW_HEIGHT / 2 + 51;
    drawing.draw_rectangle(x - 10, y - 5, width + 20, 28, Color::new(8, 7, 18, 215));
    drawing.draw_text(label, x, y, font_size, color);
}

fn draw_centered(
    drawing: &mut RaylibDrawHandle<'_>,
    text: &str,
    y: i32,
    font_size: i32,
    color: Color,
) {
    let width = drawing.measure_text(text, font_size);
    drawing.draw_text(
        text,
        (config::WINDOW_WIDTH - width) / 2,
        y,
        font_size,
        color,
    );
}
