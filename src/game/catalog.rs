use std::{error::Error, fmt};

use super::level::{Level, LevelError};

const ECLIPSE_CHAMBER_ONE: &str = include_str!("../../levels/eclipse_1.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub difficulty: &'static str,
    source: &'static str,
}

pub const LEVELS: &[LevelDefinition] = &[LevelDefinition {
    name: "Atrio del Eclipse",
    description: "Recupera la llave solar y activa el portal del templo.",
    difficulty: "INICIACION",
    source: ECLIPSE_CHAMBER_ONE,
}];

#[derive(Debug, PartialEq, Eq)]
pub enum LevelLoadError {
    UnknownIndex { index: usize, count: usize },
    InvalidLevel { index: usize, source: LevelError },
}

impl fmt::Display for LevelLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownIndex { index, count } => write!(
                formatter,
                "el nivel {} no existe; el catalogo contiene {count} niveles",
                index.saturating_add(1)
            ),
            Self::InvalidLevel { index, source } => {
                write!(
                    formatter,
                    "el nivel {} no es valido: {source}",
                    index.saturating_add(1)
                )
            }
        }
    }
}

impl Error for LevelLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidLevel { source, .. } => Some(source),
            Self::UnknownIndex { .. } => None,
        }
    }
}

pub fn level_count() -> usize {
    LEVELS.len()
}

pub fn definition(index: usize) -> Option<&'static LevelDefinition> {
    LEVELS.get(index)
}

pub fn load(index: usize) -> Result<Level, LevelLoadError> {
    let definition = definition(index).ok_or(LevelLoadError::UnknownIndex {
        index,
        count: level_count(),
    })?;

    Level::parse(definition.source).map_err(|source| LevelLoadError::InvalidLevel { index, source })
}

#[cfg(test)]
mod tests {
    use super::{LEVELS, LevelLoadError, definition, level_count, load};

    #[test]
    fn catalog_metadata_and_level_source_stay_in_sync() {
        assert!(!LEVELS.is_empty());
        assert_eq!(level_count(), LEVELS.len());

        for index in 0..level_count() {
            let entry = definition(index).expect("catalog entry");
            assert!(!entry.name.is_empty());
            assert!(!entry.description.is_empty());
            assert!(!entry.difficulty.is_empty());
            load(index).expect("every embedded level should be valid");
        }
    }

    #[test]
    fn unknown_level_index_returns_a_controlled_error() {
        assert!(matches!(
            load(level_count()),
            Err(LevelLoadError::UnknownIndex { .. })
        ));
    }
}
