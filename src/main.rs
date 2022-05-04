#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![doc = include_str!("../README.md")]

mod gui;
mod view;

fn main() {
    let a = gui::app::App::new(view::MyView::default());
    a.run();
}
