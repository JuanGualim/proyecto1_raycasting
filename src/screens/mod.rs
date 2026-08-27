use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{
    app::Screen,
    config,
    game::{
        Game,
        catalog::{self, LevelDefinition},
        level::Material,
    },
    render,
};

const NIGHT: Color = Color::new(11, 10, 24, 255);
const GOLD: Color = Color::new(239, 184, 72, 255);
const PALE_GOLD: Color = Color::new(255, 226, 156, 255);
const MUTED: Color = Color::new(171, 164, 188, 255);
const CYAN: Color = Color::new(91, 239, 207, 255);
const PANEL: Color = Color::new(18, 16, 35, 238);
const PANEL_BORDER: Color = Color::new(91, 82, 122, 255);
const EMPTY_CELL: Color = Color::new(27, 25, 43, 255);

pub fn draw(
    drawing: &mut RaylibDrawHandle<'_>,
    screen: Screen,
    selected_level: usize,
    elapsed_seconds: f32,
    game: &Game,
    level_load_error: Option<&str>,
    audio_enabled: bool,
) {
    drawing.clear_background(NIGHT);

    match screen {
        Screen::Welcome => draw_welcome(drawing, elapsed_seconds),
        Screen::LevelSelect => {
            draw_level_select(drawing, selected_level, elapsed_seconds, level_load_error)
        }
        Screen::Playing => draw_game(drawing, game),
        Screen::Paused => {
            draw_game(drawing, game);
            draw_pause(drawing);
        }
        Screen::Victory => draw_victory(drawing, game),
    }

    draw_audio_status(drawing, screen, audio_enabled);
}

fn draw_audio_status(drawing: &mut RaylibDrawHandle<'_>, screen: Screen, audio_enabled: bool) {
    let label = if audio_enabled {
        "F1  AUDIO ACTIVO"
    } else {
        "F1  AUDIO SILENCIADO"
    };
    let font_size = 14;
    let width = drawing.measure_text(label, font_size);
    let (x, y) = if matches!(screen, Screen::Playing | Screen::Paused) {
        (
            config::WINDOW_WIDTH - width - 20,
            config::WINDOW_HEIGHT - 27,
        )
    } else {
        (14, 14)
    };
    drawing.draw_rectangle(x - 7, y - 4, width + 14, 22, Color::new(8, 7, 18, 210));
    drawing.draw_text(
        label,
        x,
        y,
        font_size,
        if audio_enabled { CYAN } else { MUTED },
    );
}

fn draw_welcome(drawing: &mut RaylibDrawHandle<'_>, elapsed_seconds: f32) {
    draw_starfield(drawing, elapsed_seconds);
    for radius in [92.0, 78.0, 65.0] {
        drawing.draw_circle_lines(
            config::WINDOW_WIDTH / 2,
            128,
            radius,
            Color::new(143, 104, 62, 115),
        );
    }
    drawing.draw_circle(config::WINDOW_WIDTH / 2, 128, 54.0, GOLD);
    drawing.draw_circle(config::WINDOW_WIDTH / 2 + 18, 120, 55.0, NIGHT);

    draw_centered(drawing, "TEMPLO DEL ECLIPSE", 221, 44, GOLD);
    draw_centered(drawing, "UN RAY CASTER ESCRITO EN RUST", 276, 18, PALE_GOLD);
    draw_centered(
        drawing,
        "3 CAMARAS  |  5 MATERIALES  |  UN PORTAL",
        311,
        16,
        MUTED,
    );

    let pulse = ((elapsed_seconds * 3.0).sin() * 20.0 + 42.0) as u8;
    drawing.draw_rectangle(334, 356, 292, 54, Color::new(239, 184, 72, pulse));
    drawing.draw_rectangle_lines(334, 356, 292, 54, GOLD);

    if (elapsed_seconds * 2.0) as i32 % 2 == 0 {
        draw_centered(drawing, "ENTER  -  COMENZAR", 372, 21, Color::WHITE);
    }

    draw_centered(drawing, "Q  -  SALIR", 443, 16, MUTED);
    draw_phase_badge(drawing);
}

fn draw_level_select(
    drawing: &mut RaylibDrawHandle<'_>,
    selected_level: usize,
    elapsed_seconds: f32,
    level_load_error: Option<&str>,
) {
    draw_starfield(drawing, elapsed_seconds);
    draw_centered(drawing, "SELECCION DE CAMARA", 25, 31, GOLD);

    let Some(definition) = catalog::definition(selected_level) else {
        draw_centered(drawing, "NO SE ENCONTRO EL NIVEL", 240, 24, Color::RED);
        return;
    };

    draw_panel(drawing, 44, 76, 362, 405);
    draw_panel(drawing, 428, 76, 488, 405);
    drawing.draw_text("PLANO DEL TEMPLO", 67, 94, 16, MUTED);
    draw_level_preview(drawing, definition, 65, 121, 320, 320);
    drawing.draw_text("S  INICIO", 67, 451, 14, PALE_GOLD);
    drawing.draw_text("K  LLAVE", 163, 451, 14, GOLD);
    drawing.draw_text("G  GUARDIAN", 247, 451, 14, Color::new(255, 103, 96, 255));

    drawing.draw_text(
        &format!(
            "CAMARA {:02} / {:02}",
            selected_level + 1,
            catalog::level_count()
        ),
        458,
        101,
        17,
        MUTED,
    );
    drawing.draw_text(definition.name, 458, 135, 28, PALE_GOLD);

    let badge_width = drawing.measure_text(definition.difficulty, 15) + 24;
    drawing.draw_rectangle(458, 178, badge_width, 28, Color::new(60, 40, 80, 255));
    drawing.draw_rectangle_lines(458, 178, badge_width, 28, GOLD);
    drawing.draw_text(definition.difficulty, 470, 184, 15, GOLD);

    drawing.draw_text("MISION", 458, 226, 15, MUTED);
    draw_wrapped_text(
        drawing,
        definition.description,
        458,
        250,
        420,
        18,
        23,
        Color::WHITE,
    );

    drawing.draw_line(458, 305, 885, 305, PANEL_BORDER);
    drawing.draw_text("OBJETIVOS", 458, 320, 15, MUTED);
    draw_selector_objective(drawing, 458, 348, GOLD, "RECUPERAR LA LLAVE SOLAR");
    draw_selector_objective(
        drawing,
        458,
        378,
        Color::new(255, 103, 96, 255),
        "DERROTAR AL GUARDIAN",
    );
    draw_selector_objective(drawing, 458, 408, CYAN, "ACTIVAR Y CRUZAR EL PORTAL");

    drawing.draw_text("A / <-  CAMBIAR  -> / D", 458, 449, 16, PALE_GOLD);
    drawing.draw_text("ENTER  ENTRAR", 755, 449, 16, Color::WHITE);
    draw_level_indicators(drawing, selected_level);

    if let Some(error) = level_load_error {
        draw_centered(drawing, error, 492, 15, Color::new(255, 120, 105, 255));
    } else {
        draw_centered(drawing, "ESC  VOLVER   |   Q  SALIR", 492, 15, MUTED);
    }
    draw_phase_badge(drawing);
}

fn draw_panel(drawing: &mut RaylibDrawHandle<'_>, x: i32, y: i32, width: i32, height: i32) {
    drawing.draw_rectangle(x, y, width, height, PANEL);
    drawing.draw_rectangle_lines(x, y, width, height, PANEL_BORDER);
}

fn draw_level_preview(
    drawing: &mut RaylibDrawHandle<'_>,
    definition: &LevelDefinition,
    x: i32,
    y: i32,
    available_width: i32,
    available_height: i32,
) {
    let layout = definition.layout();
    let width = layout.lines().next().map_or(0, |row| row.chars().count());
    let height = layout.lines().count();
    if width == 0 || height == 0 {
        return;
    }

    let cell_size = (available_width / width as i32)
        .min(available_height / height as i32)
        .max(1);
    let map_width = width as i32 * cell_size;
    let map_height = height as i32 * cell_size;
    let map_x = x + (available_width - map_width) / 2;
    let map_y = y + (available_height - map_height) / 2;

    drawing.draw_rectangle(map_x - 4, map_y - 4, map_width + 8, map_height + 8, NIGHT);
    for (row, line) in layout.lines().enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            let cell_x = map_x + column as i32 * cell_size;
            let cell_y = map_y + row as i32 * cell_size;
            let color = Material::from_symbol(symbol)
                .map(render::palette::material_color)
                .unwrap_or(EMPTY_CELL);
            drawing.draw_rectangle(cell_x, cell_y, cell_size - 1, cell_size - 1, color);
            draw_preview_marker(drawing, symbol, cell_x, cell_y, cell_size);
        }
    }
    drawing.draw_rectangle_lines(map_x - 1, map_y - 1, map_width + 1, map_height + 1, MUTED);
}

fn draw_preview_marker(
    drawing: &mut RaylibDrawHandle<'_>,
    symbol: char,
    cell_x: i32,
    cell_y: i32,
    cell_size: i32,
) {
    let center_x = cell_x + cell_size / 2;
    let center_y = cell_y + cell_size / 2;
    let radius = (cell_size as f32 * 0.3).max(2.0);

    match symbol {
        'S' => {
            drawing.draw_circle(center_x, center_y, radius, PALE_GOLD);
            drawing.draw_line(center_x, center_y, center_x + cell_size / 2, center_y, GOLD);
        }
        'K' => {
            drawing.draw_circle(center_x - 2, center_y, radius * 0.55, GOLD);
            drawing.draw_line(center_x, center_y, center_x + cell_size / 3, center_y, GOLD);
        }
        'G' => {
            drawing.draw_circle(center_x, center_y, radius, Color::new(255, 82, 82, 255));
            drawing.draw_circle(center_x, center_y, radius * 0.38, NIGHT);
        }
        'E' => {
            drawing.draw_circle(center_x, center_y, radius, CYAN);
            drawing.draw_circle(center_x, center_y, radius * 0.55, NIGHT);
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_wrapped_text(
    drawing: &mut RaylibDrawHandle<'_>,
    text: &str,
    x: i32,
    y: i32,
    max_width: i32,
    font_size: i32,
    line_height: i32,
    color: Color,
) {
    let mut line = String::new();
    let mut line_y = y;

    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if !line.is_empty() && drawing.measure_text(&candidate, font_size) > max_width {
            drawing.draw_text(&line, x, line_y, font_size, color);
            line.clear();
            line.push_str(word);
            line_y += line_height;
        } else {
            line = candidate;
        }
    }

    if !line.is_empty() {
        drawing.draw_text(&line, x, line_y, font_size, color);
    }
}

fn draw_selector_objective(
    drawing: &mut RaylibDrawHandle<'_>,
    x: i32,
    y: i32,
    color: Color,
    label: &str,
) {
    drawing.draw_circle(x + 8, y + 8, 6.0, color);
    drawing.draw_circle(x + 8, y + 8, 2.0, NIGHT);
    drawing.draw_text(label, x + 25, y, 16, Color::WHITE);
}

fn draw_level_indicators(drawing: &mut RaylibDrawHandle<'_>, selected_level: usize) {
    let spacing = 18;
    let total_width = (catalog::level_count().saturating_sub(1) as i32) * spacing;
    let start_x = config::WINDOW_WIDTH / 2 - total_width / 2;

    for index in 0..catalog::level_count() {
        drawing.draw_circle(
            start_x + index as i32 * spacing,
            511,
            if index == selected_level { 5.0 } else { 3.0 },
            if index == selected_level { GOLD } else { MUTED },
        );
    }
}

fn draw_starfield(drawing: &mut RaylibDrawHandle<'_>, elapsed_seconds: f32) {
    for index in 0..52 {
        let x = (index * 137 + 53) % config::WINDOW_WIDTH;
        let y = (index * 71 + 29) % config::WINDOW_HEIGHT;
        let bright = (elapsed_seconds * 2.0 + index as f32 * 0.61).sin() > 0.35;
        drawing.draw_circle(
            x,
            y,
            if bright { 1.5 } else { 1.0 },
            if bright {
                Color::new(255, 226, 156, 150)
            } else {
                Color::new(115, 105, 144, 90)
            },
        );
    }
}

fn draw_game(drawing: &mut RaylibDrawHandle<'_>, game: &Game) {
    let depth_buffer = render::world::draw(drawing, game);
    render::sprites::draw(drawing, game, &depth_buffer);
    render::weapon::draw(drawing, game);

    drawing.draw_rectangle(12, 12, 445, 101, Color::new(8, 7, 18, 220));
    let level_name = catalog::definition(game.level_index())
        .map_or("CAMARA DESCONOCIDA", |definition| definition.name);
    drawing.draw_text(
        &format!("{}  |  RAY CASTING DDA", level_name),
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
    draw_centered(drawing, "TEMPLO PURIFICADO", 95, 46, GOLD);
    if let Some(definition) = catalog::definition(game.level_index()) {
        draw_centered(drawing, definition.name, 157, 23, CYAN);
    }
    draw_centered(
        drawing,
        "Llave recuperada  |  Guardian neutralizado  |  Portal activado",
        205,
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
    let label = "TRES NIVELES JUGABLES - FASE 4";
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
