use super::SLEEP;
use crate::widgets::{FancyHorSlider, HollowRoundToggle, RoundToggle, Toggle};
use fltk::{enums::*, prelude::*, *};
use std::sync::atomic::Ordering;
use fltk_grid::Grid;

fn fill_grid(grid: &mut Grid) {
    let mut f = frame::Frame::default().with_label("Turn on settings:");
    f.set_label_color(Color::White);
    grid.insert_ext(&mut f, 2,2, 3, 1);
    let mut g = group::Group::default().with_size(60, 30);
    Toggle::new(0, 0, 60, 15);
    g.end();
    grid.insert_ext(&mut g, 2, 16, 2, 1);
    // let mut f = frame::Frame::default().with_label("Sleep duration (millis):");
    // f.set_label_color(Color::White);
    // grid.insert_ext(&mut f, 4, 2, 3, 1);
    // // let mut g = group::Group::default().with_size(30, 10);
    // let mut slider = FancyHorSlider::new(10, 10, 30, 5);
    // grid.insert_ext(&mut *slider, 4, 14, 4, 1);
    // // 
    // let val = SLEEP.load(Ordering::Relaxed);
    // let mut f = frame::Frame::default()
    //     .with_size(0, 40)
    //     .with_label(&val.to_string());
    // f.set_label_color(Color::White);
    // grid.insert_ext(&mut f, 5, 15, 2, 1);
    // slider.set_value(val as f64 / 1000.);
    // slider.set_callback(move |s| {
    //     let val = (s.value() * 1000.) as u64 + 100;
    //     f.set_label(&val.to_string());
    //     SLEEP.store(val, Ordering::Relaxed);
    // });
    // let mut slider = FancyHorSlider::new(200, 200, 200, 10);
    // let mut win = unsafe {
    //     let mut win = window::Window::from_widget_ptr(app::first_window().unwrap().as_widget_ptr());
    //     win.assume_derived();
    //     win
    // };
    // let opacity = win.opacity();
    // let mut f = frame::Frame::default()
    //     .with_size(0, 40)
    //     .with_label(&((opacity * 100.) as i32).to_string());
    // f.set_label_color(Color::White);
    // slider.set_value(opacity);
    // slider.set_callback(move |s| {
    //     let val = s.value();
    //     f.set_label(&((val* 100.) as i32).to_string());
    //     win.set_opacity(val);
    // });
}

pub fn settings() -> group::Pack {
    let mut grp = group::Pack::default_fill()
        .center_of_parent();
    grp.set_spacing(0);
    let mut grid = Grid::default_fill();
    grid.set_layout(20, 20);
    // grid.debug(true);
    grp.end();
    fill_grid(&mut grid);
    grp
}
