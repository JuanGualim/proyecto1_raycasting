use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{app::Screen, config, game::Game, render};

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
) {
    drawing.clear_background(NIGHT);

    match screen {
        Screen::Welcome => draw_welcome(drawing, elapsed_seconds),
        Screen::LevelSelect => draw_level_select(drawing, selected_level),
        Screen::Playing => draw_game(drawing, selected_level, game),
        Screen::Paused => {
            draw_game(drawing, selected_level, game);
            draw_pause(drawing);
        }
        Screen::Victory => draw_victory(drawing),
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

fn draw_level_select(drawing: &mut RaylibDrawHandle<'_>, selected_level: usize) {
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
        "Un nivel disponible  |  ENTER para entrar",
        325,
        20,
        MUTED,
    );
    draw_centered(drawing, "Q  -  salir", 380, 18, MUTED);
    draw_phase_badge(drawing);
}

fn draw_game(drawing: &mut RaylibDrawHandle<'_>, selected_level: usize, game: &Game) {
    render::world::draw(drawing, game);

    drawing.draw_rectangle(12, 12, 380, 58, Color::new(8, 7, 18, 210));
    drawing.draw_text(
        &format!("CAMARA {}  |  RAY CASTING DDA", selected_level + 1),
        24,
        23,
        18,
        PALE_GOLD,
    );
    drawing.draw_text(
        "WASD mover  |  Mouse girar  |  ESC pausa",
        24,
        47,
        15,
        MUTED,
    );

    draw_material_legend(drawing);
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

fn draw_victory(drawing: &mut RaylibDrawHandle<'_>) {
    draw_centered(drawing, "CAMARA COMPLETADA", 170, 42, GOLD);
    draw_centered(
        drawing,
        "La condicion real de victoria llegara en la Fase 3",
        250,
        20,
        PALE_GOLD,
    );
    draw_centered(
        drawing,
        "ENTER  -  volver al selector",
        330,
        21,
        Color::WHITE,
    );
}

fn draw_phase_badge(drawing: &mut RaylibDrawHandle<'_>) {
    let label = "NAVEGACION - FASE 2.2";
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
    const MATERIALS: [(&str, Color); 5] = [
        ("1 Piedra", Color::new(102, 126, 151, 255)),
        ("2 Obsidiana", Color::new(121, 72, 181, 255)),
        ("3 Ladrillo", Color::new(190, 69, 67, 255)),
        ("4 Glifos", Color::new(224, 169, 55, 255)),
        ("5 Musgo", Color::new(63, 153, 93, 255)),
    ];

    let panel_x = config::WINDOW_WIDTH - 152;
    drawing.draw_rectangle(panel_x, 12, 140, 120, Color::new(8, 7, 18, 205));

    for (index, (label, color)) in MATERIALS.iter().enumerate() {
        let y = 22 + index as i32 * 21;
        drawing.draw_rectangle(panel_x + 10, y, 12, 12, *color);
        drawing.draw_text(label, panel_x + 29, y - 1, 14, Color::WHITE);
    }
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
