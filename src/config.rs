pub const WINDOW_WIDTH: i32 = 960;
pub const WINDOW_HEIGHT: i32 = 540;
pub const WINDOW_TITLE: &str = "Templo del Eclipse";
pub const TARGET_FPS: u32 = 60;

/// Limita los saltos de simulacion al recuperar el foco o salir de una pausa.
pub const MAX_DELTA_TIME: f32 = 1.0 / 20.0;
pub const PLAYER_MOVE_SPEED: f32 = 3.0;
pub const PLAYER_RADIUS: f32 = 0.22;
pub const MOUSE_SENSITIVITY: f32 = 0.0025;
pub const CAMERA_PLANE_LENGTH: f32 = 0.66;

pub const SHOT_COOLDOWN: f32 = 0.28;
pub const MUZZLE_FLASH_DURATION: f32 = 0.075;
pub const SHOT_FEEDBACK_DURATION: f32 = 0.16;
pub const GUARDIAN_HIT_FLASH_DURATION: f32 = 0.13;
pub const GUARDIAN_HIT_RADIUS: f32 = 0.34;
pub const GUARDIAN_MAX_HEALTH: i32 = 3;
pub const ENTITY_INTERACTION_RADIUS: f32 = 0.48;
pub const INTERACTION_FEEDBACK_DURATION: f32 = 1.25;
pub const ENTITY_ANIMATION_FRAME_TIME: f32 = 0.16;

pub const MUSIC_VOLUME: f32 = 0.32;
pub const SOUND_EFFECT_VOLUME: f32 = 0.58;
