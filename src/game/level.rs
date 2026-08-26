use std::{error::Error, fmt};

use super::{
    entities::{EntityKind, EntitySpawn},
    math::Vec2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Material {
    Stone,
    Obsidian,
    Brick,
    Glyph,
    Moss,
}

impl Material {
    fn from_symbol(symbol: char) -> Option<Self> {
        match symbol {
            '1' => Some(Self::Stone),
            '2' => Some(Self::Obsidian),
            '3' => Some(Self::Brick),
            '4' => Some(Self::Glyph),
            '5' => Some(Self::Moss),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall(Material),
}

#[derive(Debug, PartialEq, Eq)]
pub enum LevelError {
    TooSmall,
    IrregularRow {
        row: usize,
        expected: usize,
        found: usize,
    },
    InvalidSymbol {
        symbol: char,
        column: usize,
        row: usize,
    },
    MissingSpawn,
    MultipleSpawns,
    OpenBorder {
        column: usize,
        row: usize,
    },
}

impl fmt::Display for LevelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooSmall => write!(formatter, "el mapa debe medir al menos 3 x 3"),
            Self::IrregularRow {
                row,
                expected,
                found,
            } => write!(
                formatter,
                "la fila {row} mide {found}, pero se esperaban {expected} celdas"
            ),
            Self::InvalidSymbol {
                symbol,
                column,
                row,
            } => write!(
                formatter,
                "simbolo '{symbol}' no valido en columna {column}, fila {row}"
            ),
            Self::MissingSpawn => write!(formatter, "el mapa no contiene una posicion S"),
            Self::MultipleSpawns => write!(formatter, "el mapa contiene mas de una posicion S"),
            Self::OpenBorder { column, row } => write!(
                formatter,
                "el borde debe ser solido; abertura en columna {column}, fila {row}"
            ),
        }
    }
}

impl Error for LevelError {}

pub struct Level {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    spawn: Vec2,
    entity_spawns: Vec<EntitySpawn>,
}

impl Level {
    pub fn parse(source: &str) -> Result<Self, LevelError> {
        let source = source.trim();
        if source.is_empty() {
            return Err(LevelError::TooSmall);
        }

        let rows: Vec<&str> = source.lines().map(str::trim).collect();
        let height = rows.len();
        let width = rows.first().map_or(0, |row| row.chars().count());

        if width < 3 || height < 3 {
            return Err(LevelError::TooSmall);
        }

        let mut tiles = Vec::with_capacity(width * height);
        let mut spawn = None;
        let mut entity_spawns = Vec::new();

        for (row_index, row) in rows.iter().enumerate() {
            let found_width = row.chars().count();
            if found_width != width {
                return Err(LevelError::IrregularRow {
                    row: row_index + 1,
                    expected: width,
                    found: found_width,
                });
            }

            for (column_index, symbol) in row.chars().enumerate() {
                let tile = match symbol {
                    '.' => Tile::Empty,
                    'S' => {
                        if spawn.is_some() {
                            return Err(LevelError::MultipleSpawns);
                        }
                        spawn = Some(Vec2::new(column_index as f32 + 0.5, row_index as f32 + 0.5));
                        Tile::Empty
                    }
                    symbol => {
                        if let Some(kind) = EntityKind::from_symbol(symbol) {
                            entity_spawns.push(EntitySpawn {
                                kind,
                                position: Vec2::new(
                                    column_index as f32 + 0.5,
                                    row_index as f32 + 0.5,
                                ),
                            });
                            Tile::Empty
                        } else {
                            Material::from_symbol(symbol).map(Tile::Wall).ok_or(
                                LevelError::InvalidSymbol {
                                    symbol,
                                    column: column_index + 1,
                                    row: row_index + 1,
                                },
                            )?
                        }
                    }
                };
                tiles.push(tile);
            }
        }

        let level = Self {
            width,
            height,
            tiles,
            spawn: spawn.ok_or(LevelError::MissingSpawn)?,
            entity_spawns,
        };
        level.validate_closed_border()?;

        Ok(level)
    }

    fn validate_closed_border(&self) -> Result<(), LevelError> {
        for row in 0..self.height {
            for column in 0..self.width {
                let is_border =
                    row == 0 || row == self.height - 1 || column == 0 || column == self.width - 1;

                if is_border && self.wall_material_at(column as i32, row as i32).is_none() {
                    return Err(LevelError::OpenBorder {
                        column: column + 1,
                        row: row + 1,
                    });
                }
            }
        }

        Ok(())
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn spawn(&self) -> Vec2 {
        self.spawn
    }

    pub fn entity_spawns(&self) -> &[EntitySpawn] {
        &self.entity_spawns
    }

    pub fn contains(&self, column: i32, row: i32) -> bool {
        column >= 0 && row >= 0 && (column as usize) < self.width && (row as usize) < self.height
    }

    pub fn wall_material_at(&self, column: i32, row: i32) -> Option<Material> {
        if !self.contains(column, row) {
            return None;
        }

        let index = row as usize * self.width + column as usize;
        match self.tiles[index] {
            Tile::Empty => None,
            Tile::Wall(material) => Some(material),
        }
    }

    /// El exterior se considera solido para que la colision sea segura incluso
    /// si un mapa defectuoso o una posicion extrema alcanzan sus limites.
    pub fn is_solid(&self, column: i32, row: i32) -> bool {
        !self.contains(column, row) || self.wall_material_at(column, row).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::{Level, LevelError, Material};
    use crate::game::{entities::EntityKind, math::Vec2};

    const VALID_LEVEL: &str = "\
11111
1S..2
1...2
1...2
11111";

    #[test]
    fn parses_a_closed_level_and_centers_the_spawn() {
        let level = Level::parse(VALID_LEVEL).expect("valid map");

        assert_eq!(level.width(), 5);
        assert_eq!(level.height(), 5);
        assert_eq!(level.spawn().x, 1.5);
        assert_eq!(level.spawn().y, 1.5);
        assert_eq!(level.wall_material_at(4, 2), Some(Material::Obsidian));
        assert_eq!(level.wall_material_at(2, 2), None);
        assert!(level.is_solid(-1, 2));
        assert!(level.is_solid(5, 2));
    }

    #[test]
    fn rejects_irregular_rows() {
        let error = Level::parse("1111\n1S.1\n111").err();

        assert!(matches!(error, Some(LevelError::IrregularRow { .. })));
    }

    #[test]
    fn rejects_an_open_border() {
        let error = Level::parse("1111\n1S.1\n1..1\n11.1").err();

        assert!(matches!(error, Some(LevelError::OpenBorder { .. })));
    }

    #[test]
    fn rejects_a_level_without_spawn() {
        let error = Level::parse("1111\n1..1\n1..1\n1111").err();

        assert_eq!(error, Some(LevelError::MissingSpawn));
    }

    #[test]
    fn rejects_unknown_symbols_and_multiple_spawns() {
        let invalid_symbol = Level::parse("1111\n1SX1\n1..1\n1111").err();
        let multiple_spawns = Level::parse("1111\n1SS1\n1..1\n1111").err();

        assert!(matches!(
            invalid_symbol,
            Some(LevelError::InvalidSymbol { symbol: 'X', .. })
        ));
        assert_eq!(multiple_spawns, Some(LevelError::MultipleSpawns));
    }

    #[test]
    fn parses_entity_symbols_as_walkable_centered_spawns() {
        let level = Level::parse("111111\n1SKGE1\n1....1\n111111").expect("valid entities");

        assert_eq!(level.entity_spawns().len(), 3);
        assert_eq!(level.entity_spawns()[0].kind, EntityKind::Key);
        assert_eq!(level.entity_spawns()[0].position, Vec2::new(2.5, 1.5));
        assert_eq!(level.entity_spawns()[1].kind, EntityKind::Guardian);
        assert_eq!(level.entity_spawns()[2].kind, EntityKind::Portal);
        assert!(!level.is_solid(2, 1));
        assert!(!level.is_solid(3, 1));
        assert!(!level.is_solid(4, 1));
    }
}
