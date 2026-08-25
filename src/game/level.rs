use std::{error::Error, fmt};

use super::math::Vec2;

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
                    symbol => Material::from_symbol(symbol).map(Tile::Wall).ok_or(
                        LevelError::InvalidSymbol {
                            symbol,
                            column: column_index + 1,
                            row: row_index + 1,
                        },
                    )?,
                };
                tiles.push(tile);
            }
        }

        let level = Self {
            width,
            height,
            tiles,
            spawn: spawn.ok_or(LevelError::MissingSpawn)?,
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
}

#[cfg(test)]
mod tests {
    use super::{Level, LevelError, Material};

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
}
