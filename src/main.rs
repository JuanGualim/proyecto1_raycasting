mod app;
mod config;
mod game;
mod render;
mod screens;

use app::App;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(config::WINDOW_WIDTH, config::WINDOW_HEIGHT)
        .title(config::WINDOW_TITLE)
        .build();

    window.set_target_fps(config::TARGET_FPS);
    window.set_exit_key(None);

    let mut app = match App::new() {
        Ok(app) => app,
        Err(error) => {
            eprintln!("No se pudo cargar el nivel incluido: {error}");
            return;
        }
    };

    while !window.window_should_close() && !app.should_quit() {
        let delta_time = window.get_frame_time().min(config::MAX_DELTA_TIME);
        app.update(&mut window, delta_time);

        let mut drawing = window.begin_drawing(&thread);
        app.draw(&mut drawing);
    }
}
