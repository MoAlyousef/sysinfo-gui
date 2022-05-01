#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod styles;
mod view;
mod widgets;

fn main() {
    let a = gui::app::App::new(view::MyView::default());
    a.run();
}
