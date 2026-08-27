use raylib::prelude::{KeyboardKey, MouseButton, RaylibDrawHandle, RaylibHandle};

use crate::{
    audio::AudioCue,
    config,
    game::{Game, catalog, catalog::LevelLoadError, combat::ShotOutcome, objective::GameEvent},
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
    level_load_error: Option<String>,
    audio_enabled: bool,
    audio_cues: Vec<AudioCue>,
}

impl App {
    pub fn new() -> Result<Self, LevelLoadError> {
        Ok(Self {
            screen: Screen::Welcome,
            selected_level: 0,
            elapsed_seconds: 0.0,
            should_quit: false,
            game: Game::load_first_level()?,
            level_load_error: None,
            audio_enabled: true,
            audio_cues: Vec::new(),
        })
    }

    pub fn update(&mut self, input: &mut RaylibHandle, delta_time: f32) {
        self.elapsed_seconds += delta_time;

        if input.is_key_pressed(KeyboardKey::KEY_F1) {
            self.audio_enabled = !self.audio_enabled;
            if self.audio_enabled {
                self.audio_cues.push(AudioCue::MenuConfirm);
            }
        }

        match self.screen {
            Screen::Welcome => {
                if input.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    self.transition_to(Screen::LevelSelect, input);
                }
            }
            Screen::LevelSelect => {
                if input.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    self.transition_to(Screen::Welcome, input);
                } else {
                    self.update_level_select(input);
                }
            }
            Screen::Playing => {
                if input.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    self.transition_to(Screen::Paused, input);
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
                    self.game.reset_level();
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
        let move_left =
            input.is_key_pressed(KeyboardKey::KEY_LEFT) || input.is_key_pressed(KeyboardKey::KEY_A);
        let move_right = input.is_key_pressed(KeyboardKey::KEY_RIGHT)
            || input.is_key_pressed(KeyboardKey::KEY_D);

        if move_left {
            self.selected_level = cycle_level_index(self.selected_level, -1);
            self.level_load_error = None;
            self.audio_cues.push(AudioCue::MenuMove);
        } else if move_right {
            self.selected_level = cycle_level_index(self.selected_level, 1);
            self.level_load_error = None;
            self.audio_cues.push(AudioCue::MenuMove);
        }

        if input.is_key_pressed(KeyboardKey::KEY_ENTER) {
            match self.load_selected_level() {
                Ok(()) => self.transition_to(Screen::Playing, input),
                Err(error) => self.level_load_error = Some(error.to_string()),
            }
        }
    }

    fn load_selected_level(&mut self) -> Result<(), LevelLoadError> {
        self.game = Game::load_level(self.selected_level)?;
        self.level_load_error = None;
        Ok(())
    }

    fn update_playing(&mut self, input: &mut RaylibHandle, delta_time: f32) {
        self.game.tick(delta_time);
        let rotation = input.get_mouse_delta().x * config::MOUSE_SENSITIVITY;
        self.game.rotate_player(rotation);

        if input.is_key_pressed(KeyboardKey::KEY_R) {
            self.game.reset_level();
            self.audio_cues.push(AudioCue::MenuConfirm);
        }

        if input.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let portal_was_ready = self.game.portal_ready();
            let outcome = self.game.try_shoot();
            self.record_shot_audio(outcome);
            self.record_portal_activation(portal_was_ready);
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

        let had_key = self.game.has_key();
        let portal_was_ready = self.game.portal_ready();
        let event = self.game.update_interactions();
        if !had_key && self.game.has_key() {
            self.audio_cues.push(AudioCue::KeyCollected);
        }
        self.record_portal_activation(portal_was_ready);

        if event == Some(GameEvent::Victory) {
            self.transition_to(Screen::Victory, input);
        }
    }

    fn record_shot_audio(&mut self, outcome: ShotOutcome) {
        if outcome == ShotOutcome::Cooldown {
            return;
        }

        self.audio_cues.push(AudioCue::Shot);
        match outcome {
            ShotOutcome::Hit { destroyed: false } => {
                self.audio_cues.push(AudioCue::GuardianHit);
            }
            ShotOutcome::Hit { destroyed: true } => {
                self.audio_cues.push(AudioCue::GuardianDefeated);
            }
            ShotOutcome::Miss | ShotOutcome::Blocked | ShotOutcome::Cooldown => {}
        }
    }

    fn record_portal_activation(&mut self, portal_was_ready: bool) {
        if !portal_was_ready && self.game.portal_ready() {
            self.audio_cues.push(AudioCue::PortalActivated);
        }
    }

    fn transition_to(&mut self, next_screen: Screen, input: &mut RaylibHandle) {
        if self.screen == next_screen {
            return;
        }

        let cue = match (self.screen, next_screen) {
            (_, Screen::Victory) => AudioCue::Victory,
            (Screen::Welcome, Screen::LevelSelect)
            | (Screen::LevelSelect, Screen::Playing)
            | (Screen::Paused, Screen::Playing)
            | (Screen::Victory, Screen::LevelSelect) => AudioCue::MenuConfirm,
            _ => AudioCue::MenuMove,
        };
        self.audio_cues.push(cue);

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
            self.level_load_error.as_deref(),
            self.audio_enabled,
        );
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn audio_enabled(&self) -> bool {
        self.audio_enabled
    }

    pub fn take_audio_cues(&mut self) -> Vec<AudioCue> {
        std::mem::take(&mut self.audio_cues)
    }
}

fn cycle_level_index(current: usize, direction: i32) -> usize {
    let count = catalog::level_count();
    debug_assert!(count > 0);
    let current = current % count;

    if direction < 0 {
        current.checked_sub(1).unwrap_or(count - 1)
    } else {
        (current + 1) % count
    }
}

#[cfg(test)]
mod tests {
    use super::{App, Screen, cycle_level_index};
    use crate::{
        audio::AudioCue,
        game::{catalog, combat::ShotOutcome},
    };

    #[test]
    fn application_starts_on_welcome_screen() {
        let app = App::new().expect("embedded level should be valid");

        assert_eq!(app.screen, Screen::Welcome);
        assert_eq!(app.selected_level, 0);
        assert!(app.audio_enabled());
        assert!(!app.should_quit());
    }

    #[test]
    fn selected_level_creates_a_fresh_game() {
        let mut app = App::new().expect("embedded level should be valid");
        app.game.tick(2.0);

        app.load_selected_level()
            .expect("selected level should load");

        assert_eq!(app.game.level_index(), app.selected_level);
        assert_eq!(app.game.level_elapsed_seconds(), 0.0);
        assert!(app.level_load_error.is_none());
    }

    #[test]
    fn invalid_selection_does_not_replace_the_current_game() {
        let mut app = App::new().expect("embedded level should be valid");
        let original_level = app.game.level_index();
        app.selected_level = catalog::level_count();

        assert!(app.load_selected_level().is_err());
        assert_eq!(app.game.level_index(), original_level);
    }

    #[test]
    fn level_selection_wraps_in_both_directions() {
        let last = catalog::level_count() - 1;

        assert_eq!(cycle_level_index(0, -1), last);
        assert_eq!(cycle_level_index(last, 1), 0);
        assert_eq!(cycle_level_index(0, 1), 1);
    }

    #[test]
    fn combat_audio_ignores_cooldown_and_distinguishes_hits() {
        let mut app = App::new().expect("embedded level should be valid");

        app.record_shot_audio(ShotOutcome::Cooldown);
        assert!(app.take_audio_cues().is_empty());

        app.record_shot_audio(ShotOutcome::Miss);
        assert_eq!(app.take_audio_cues(), [AudioCue::Shot]);

        app.record_shot_audio(ShotOutcome::Hit { destroyed: false });
        assert_eq!(
            app.take_audio_cues(),
            [AudioCue::Shot, AudioCue::GuardianHit]
        );

        app.record_shot_audio(ShotOutcome::Hit { destroyed: true });
        assert_eq!(
            app.take_audio_cues(),
            [AudioCue::Shot, AudioCue::GuardianDefeated]
        );
    }
}
