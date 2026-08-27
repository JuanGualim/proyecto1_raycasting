mod app;
mod audio;
mod config;
mod game;
mod render;
mod screens;

use app::App;
use audio::AudioSystem;
use raylib::prelude::RaylibAudio;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(config::WINDOW_WIDTH, config::WINDOW_HEIGHT)
        .title(config::WINDOW_TITLE)
        .build();

    window.set_target_fps(config::TARGET_FPS);
    window.set_exit_key(None);

    let audio_device = match RaylibAudio::init_audio_device() {
        Ok(device) => Some(device),
        Err(error) => {
            eprintln!("El juego continuara sin audio: {error}");
            None
        }
    };
    let mut audio_system = audio_device.as_ref().and_then(|device| {
        AudioSystem::new(device)
            .map_err(|error| eprintln!("El juego continuara sin musica: {error}"))
            .ok()
    });

    let mut app = match App::new() {
        Ok(app) => app,
        Err(error) => {
            eprintln!("No se pudo cargar el catalogo de niveles: {error}");
            return;
        }
    };

    while !window.window_should_close() && !app.should_quit() {
        let delta_time = window.get_frame_time().min(config::MAX_DELTA_TIME);
        app.update(&mut window, delta_time);
        let audio_enabled = app.audio_enabled();
        let audio_cues = app.take_audio_cues();
        if let Some(audio) = &mut audio_system {
            audio.update(audio_enabled);
            for cue in audio_cues {
                audio.play(cue);
            }
        }

        let mut drawing = window.begin_drawing(&thread);
        app.draw(&mut drawing);
    }
}
