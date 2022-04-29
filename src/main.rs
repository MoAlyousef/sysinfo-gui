#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod gui;
mod logic;
mod styles;
mod view;
mod widgets;

fn main() {
    std::panic::set_hook(Box::new(|_| {
        // do nothing
    }));
    logic::background_thread_spawn();
    gui::app::App::new().run();
}
