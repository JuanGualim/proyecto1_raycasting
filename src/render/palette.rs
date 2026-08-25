use raylib::prelude::Color;

use crate::game::level::Material;

pub const fn material_rgb(material: Material) -> (u8, u8, u8) {
    match material {
        Material::Stone => (102, 126, 151),
        Material::Obsidian => (121, 72, 181),
        Material::Brick => (190, 69, 67),
        Material::Glyph => (224, 169, 55),
        Material::Moss => (63, 153, 93),
    }
}

pub const fn material_color(material: Material) -> Color {
    let (red, green, blue) = material_rgb(material);
    Color::new(red, green, blue, 255)
}
