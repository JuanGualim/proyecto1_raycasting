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
    pub animation_time: f32,
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
            animation_time: 0.0,
        }
    }
}

impl Entity {
    pub fn tick(&mut self, delta_time: f32) {
        self.hit_flash_remaining = (self.hit_flash_remaining - delta_time).max(0.0);
        if self.active {
            self.animation_time = (self.animation_time + delta_time).rem_euclid(60.0);
        }
    }

    pub fn animation_frame(&self) -> usize {
        let frame_count = match self.kind {
            EntityKind::Key | EntityKind::Guardian => 4,
            EntityKind::Portal => 6,
        };
        ((self.animation_time / crate::config::ENTITY_ANIMATION_FRAME_TIME).floor() as usize)
            % frame_count
    }
}

#[cfg(test)]
mod tests {
    use super::{Entity, EntityKind, EntitySpawn};
    use crate::game::math::Vec2;

    #[test]
    fn active_entity_advances_animation_frames() {
        let mut entity = Entity::from(EntitySpawn {
            kind: EntityKind::Guardian,
            position: Vec2::new(2.5, 2.5),
        });

        assert_eq!(entity.animation_frame(), 0);
        entity.tick(crate::config::ENTITY_ANIMATION_FRAME_TIME);
        assert_eq!(entity.animation_frame(), 1);
        entity.tick(crate::config::ENTITY_ANIMATION_FRAME_TIME * 3.0);
        assert_eq!(entity.animation_frame(), 0);
    }

    #[test]
    fn inactive_entity_does_not_advance_animation() {
        let mut entity = Entity::from(EntitySpawn {
            kind: EntityKind::Key,
            position: Vec2::new(2.5, 2.5),
        });
        entity.active = false;

        entity.tick(1.0);

        assert_eq!(entity.animation_frame(), 0);
    }
}
