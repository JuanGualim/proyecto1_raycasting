pub const WINDOW_WIDTH: i32 = 960;
pub const WINDOW_HEIGHT: i32 = 540;
pub const WINDOW_TITLE: &str = "Templo del Eclipse";
pub const TARGET_FPS: u32 = 60;

/// Limita los saltos de simulacion al recuperar el foco o salir de una pausa.
pub const MAX_DELTA_TIME: f32 = 1.0 / 20.0;
/// La seleccion multiple se habilitara en la Fase 4.
pub const LEVEL_COUNT: usize = 1;

pub const PLAYER_MOVE_SPEED: f32 = 3.0;
pub const PLAYER_RADIUS: f32 = 0.22;
pub const MOUSE_SENSITIVITY: f32 = 0.0025;
pub const CAMERA_PLANE_LENGTH: f32 = 0.66;
