use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, Rectangle};

use crate::{config, game::Game};

use super::palette;

const MARGIN: f32 = 12.0;
const PADDING: f32 = 8.0;
const HEADER_HEIGHT: f32 = 24.0;
const MAX_MAP_SIZE: f32 = 144.0;
const MAX_CELL_SIZE: f32 = 9.0;
const PANEL_BACKGROUND: Color = Color::new(8, 7, 18, 220);
const EMPTY_CELL: Color = Color::new(23, 21, 34, 205);
const BORDER: Color = Color::new(137, 126, 159, 235);
const PLAYER: Color = Color::new(255, 226, 112, 255);
const VIEW_CONE: Color = Color::new(255, 210, 92, 180);

#[derive(Debug, Clone, Copy)]
struct MinimapLayout {
    panel_x: f32,
    panel_y: f32,
    panel_width: f32,
    panel_height: f32,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    cell_size: f32,
}

impl MinimapLayout {
    fn for_dimensions(width: usize, height: usize) -> Self {
        let largest_dimension = width.max(height) as f32;
        let cell_size = (MAX_MAP_SIZE / largest_dimension).min(MAX_CELL_SIZE);
        let map_width = width as f32 * cell_size;
        let map_height = height as f32 * cell_size;
        let panel_width = map_width + PADDING * 2.0;
        let panel_height = map_height + HEADER_HEIGHT + PADDING;
        let panel_x = config::WINDOW_WIDTH as f32 - panel_width - MARGIN;
        let panel_y = MARGIN;

        Self {
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            map_x: panel_x + PADDING,
            map_y: panel_y + HEADER_HEIGHT,
            map_width,
            map_height,
            cell_size,
        }
    }

    fn project(self, world_x: f32, world_y: f32) -> (f32, f32) {
        (
            self.map_x + world_x * self.cell_size,
            self.map_y + world_y * self.cell_size,
        )
    }
}

pub fn draw(drawing: &mut RaylibDrawHandle<'_>, game: &Game) {
    let level = game.level();
    let player = game.player();
    let layout = MinimapLayout::for_dimensions(level.width(), level.height());

    drawing.draw_rectangle(
        layout.panel_x.floor() as i32,
        layout.panel_y.floor() as i32,
        layout.panel_width.ceil() as i32,
        layout.panel_height.ceil() as i32,
        PANEL_BACKGROUND,
    );
    drawing.draw_rectangle_lines(
        layout.panel_x.floor() as i32,
        layout.panel_y.floor() as i32,
        layout.panel_width.ceil() as i32,
        layout.panel_height.ceil() as i32,
        BORDER,
    );
    drawing.draw_text(
        &format!(
            "MAPA  {:04.1}, {:04.1}",
            player.position.x, player.position.y
        ),
        (layout.panel_x + PADDING) as i32,
        (layout.panel_y + 5.0) as i32,
        13,
        Color::WHITE,
    );

    for row in 0..level.height() {
        for column in 0..level.width() {
            let color = level
                .wall_material_at(column as i32, row as i32)
                .map_or(EMPTY_CELL, palette::material_color);
            let inset = if layout.cell_size >= 3.0 { 0.6 } else { 0.0 };
            drawing.draw_rectangle_rec(
                Rectangle::new(
                    layout.map_x + column as f32 * layout.cell_size + inset * 0.5,
                    layout.map_y + row as f32 * layout.cell_size + inset * 0.5,
                    layout.cell_size - inset,
                    layout.cell_size - inset,
                ),
                color,
            );
        }
    }
    drawing.draw_rectangle_lines(
        layout.map_x.floor() as i32,
        layout.map_y.floor() as i32,
        layout.map_width.ceil() as i32,
        layout.map_height.ceil() as i32,
        BORDER,
    );

    let (player_x, player_y) = layout.project(player.position.x, player.position.y);
    let view_length = layout.cell_size * 1.7;
    let left_x = player.direction.x - player.camera_plane.x;
    let left_y = player.direction.y - player.camera_plane.y;
    let right_x = player.direction.x + player.camera_plane.x;
    let right_y = player.direction.y + player.camera_plane.y;

    draw_direction_line(
        drawing,
        player_x,
        player_y,
        left_x,
        left_y,
        view_length,
        VIEW_CONE,
    );
    draw_direction_line(
        drawing,
        player_x,
        player_y,
        right_x,
        right_y,
        view_length,
        VIEW_CONE,
    );
    draw_direction_line(
        drawing,
        player_x,
        player_y,
        player.direction.x,
        player.direction.y,
        view_length * 1.15,
        PLAYER,
    );
    drawing.draw_circle(
        player_x.round() as i32,
        player_y.round() as i32,
        (layout.cell_size * 0.35).max(2.5),
        PLAYER,
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_direction_line(
    drawing: &mut RaylibDrawHandle<'_>,
    start_x: f32,
    start_y: f32,
    direction_x: f32,
    direction_y: f32,
    length: f32,
    color: Color,
) {
    drawing.draw_line(
        start_x.round() as i32,
        start_y.round() as i32,
        (start_x + direction_x * length).round() as i32,
        (start_y + direction_y * length).round() as i32,
        color,
    );
}

#[cfg(test)]
mod tests {
    use super::{MAX_MAP_SIZE, MinimapLayout};

    #[test]
    fn layout_stays_in_the_top_right_corner() {
        let layout = MinimapLayout::for_dimensions(16, 16);

        assert!(layout.panel_x > 0.0);
        assert_eq!(layout.panel_y, 12.0);
        assert!(layout.map_width <= MAX_MAP_SIZE);
        assert!(layout.map_height <= MAX_MAP_SIZE);
        assert!(layout.panel_x + layout.panel_width <= 960.0);
        assert!(layout.panel_y + layout.panel_height <= 540.0);
    }

    #[test]
    fn world_coordinates_are_projected_inside_the_map() {
        let layout = MinimapLayout::for_dimensions(16, 16);
        let (left, top) = layout.project(0.0, 0.0);
        let (right, bottom) = layout.project(16.0, 16.0);

        assert_eq!(left, layout.map_x);
        assert_eq!(top, layout.map_y);
        assert_eq!(right, layout.map_x + layout.map_width);
        assert_eq!(bottom, layout.map_y + layout.map_height);
    }
}
