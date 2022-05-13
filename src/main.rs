#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![doc = include_str!("../README.md")]

mod gui;
mod view;

fn main() {
    gui::app::App::new().run(view::MyView::default());
}
