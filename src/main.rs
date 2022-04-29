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
    let a = gui::app::App::new(view::MyView::default());
    a.spawn(logic::background_thread_spawn);
    a.run();
}
