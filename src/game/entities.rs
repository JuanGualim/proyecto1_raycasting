use super::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Key,
    Portal,
    Guardian,
}

impl EntityKind {
    pub fn from_symbol(symbol: char) -> Option<Self> {
        match symbol {
            'K' => Some(Self::Key),
            'E' => Some(Self::Portal),
            'G' => Some(Self::Guardian),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntitySpawn {
    pub kind: EntityKind,
    pub position: Vec2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub kind: EntityKind,
    pub position: Vec2,
    pub active: bool,
    pub health: i32,
    pub hit_flash_remaining: f32,
}

impl From<EntitySpawn> for Entity {
    fn from(spawn: EntitySpawn) -> Self {
        Self {
            kind: spawn.kind,
            position: spawn.position,
            active: true,
            health: match spawn.kind {
                EntityKind::Guardian => crate::config::GUARDIAN_MAX_HEALTH,
                EntityKind::Key | EntityKind::Portal => 0,
            },
            hit_flash_remaining: 0.0,
        }
    }
}
