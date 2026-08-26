use raylib::prelude::{KeyboardKey, MouseButton, RaylibDrawHandle, RaylibHandle};

use crate::{
    config,
    game::{Game, level::LevelError},
    screens,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Welcome,
    LevelSelect,
    Playing,
    Paused,
    Victory,
}

pub struct App {
    screen: Screen,
    selected_level: usize,
    elapsed_seconds: f32,
    should_quit: bool,
    game: Game,
}

impl App {
    pub fn new() -> Result<Self, LevelError> {
        Ok(Self {
            screen: Screen::Welcome,
            selected_level: 0,
            elapsed_seconds: 0.0,
            should_quit: false,
            game: Game::load_first_level()?,
        })
    }

    pub fn update(&mut self, input: &mut RaylibHandle, delta_time: f32) {
        self.elapsed_seconds += delta_time;

        match self.screen {
            Screen::Welcome => {
                if input.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    self.transition_to(Screen::LevelSelect, input);
                }
            }
            Screen::LevelSelect => self.update_level_select(input),
            Screen::Playing => {
                if input.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    self.transition_to(Screen::Paused, input);
                } else if input.is_key_pressed(KeyboardKey::KEY_V) {
                    // Transicion temporal para comprobar el flujo de pantallas.
                    self.transition_to(Screen::Victory, input);
                } else {
                    self.update_playing(input, delta_time);
                }
            }
            Screen::Paused => {
                if input.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    self.transition_to(Screen::Playing, input);
                } else if input.is_key_pressed(KeyboardKey::KEY_M) {
                    self.transition_to(Screen::LevelSelect, input);
                }
            }
            Screen::Victory => {
                if input.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    self.transition_to(Screen::LevelSelect, input);
                }
            }
        }

        if input.is_key_pressed(KeyboardKey::KEY_Q)
            && !matches!(self.screen, Screen::Playing | Screen::Paused)
        {
            self.should_quit = true;
        }
    }

    fn update_level_select(&mut self, input: &mut RaylibHandle) {
        if input.is_key_pressed(KeyboardKey::KEY_LEFT) {
            self.selected_level = self
                .selected_level
                .checked_sub(1)
                .unwrap_or(config::LEVEL_COUNT - 1);
        }

        if input.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            self.selected_level = if self.selected_level + 1 >= config::LEVEL_COUNT {
                0
            } else {
                self.selected_level + 1
            };
        }

        if input.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.transition_to(Screen::Playing, input);
        }
    }

    fn update_playing(&mut self, input: &RaylibHandle, delta_time: f32) {
        self.game.tick(delta_time);
        let rotation = input.get_mouse_delta().x * config::MOUSE_SENSITIVITY;
        self.game.rotate_player(rotation);

        if input.is_key_pressed(KeyboardKey::KEY_R) {
            self.game.reset_level();
        }

        if input.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            self.game.try_shoot();
        }

        let mut forward_axis = 0.0;
        let mut strafe_axis = 0.0;

        if input.is_key_down(KeyboardKey::KEY_W) {
            forward_axis += 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_S) {
            forward_axis -= 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_D) {
            strafe_axis += 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_A) {
            strafe_axis -= 1.0;
        }

        self.game.move_player(forward_axis, strafe_axis, delta_time);
    }

    fn transition_to(&mut self, next_screen: Screen, input: &mut RaylibHandle) {
        let was_captured = self.screen == Screen::Playing;
        let should_capture = next_screen == Screen::Playing;

        if !was_captured && should_capture {
            input.disable_cursor();
        } else if was_captured && !should_capture {
            input.enable_cursor();
        }

        self.screen = next_screen;
    }

    pub fn draw(&self, drawing: &mut RaylibDrawHandle<'_>) {
        screens::draw(
            drawing,
            self.screen,
            self.selected_level,
            self.elapsed_seconds,
            &self.game,
        );
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

#[cfg(test)]
mod tests {
    use super::{App, Screen};

    #[test]
    fn application_starts_on_welcome_screen() {
        let app = App::new().expect("embedded level should be valid");

        assert_eq!(app.screen, Screen::Welcome);
        assert_eq!(app.selected_level, 0);
        assert!(!app.should_quit());
    }
}
