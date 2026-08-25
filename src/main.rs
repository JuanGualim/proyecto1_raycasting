mod app;
mod config;
mod screens;

use app::App;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(config::WINDOW_WIDTH, config::WINDOW_HEIGHT)
        .title(config::WINDOW_TITLE)
        .build();

    window.set_target_fps(config::TARGET_FPS);
    window.set_exit_key(None);

    let mut app = App::new();

    while !window.window_should_close() && !app.should_quit() {
        let delta_time = window.get_frame_time().min(config::MAX_DELTA_TIME);
        app.update(&window, delta_time);

        let mut drawing = window.begin_drawing(&thread);
        app.draw(&mut drawing);
    }
}
