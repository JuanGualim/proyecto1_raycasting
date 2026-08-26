#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
    Victory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionFeedback {
    KeyCollected,
    PortalNeedsKey,
    PortalNeedsGuardian,
}
