use super::{LIGHT_MODE, SLEEP};
use crate::styles::colors::*;
use crate::widgets::{FancyHorSlider, Toggle};
use fltk::{enums::*, prelude::*, *};
use fltk_grid::Grid;
use std::sync::atomic::Ordering;

fn fill_grid(grid: &mut Grid) {
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_label("Light mode:");
    grid.insert_ext(&mut f, 3, 2, 3, 1);
    let mut g = group::Group::default().with_size(60, 30);
    let mut t = Toggle::new(0, 0, 60, 15).center_of_parent();
    t.set_value(LIGHT_MODE.load(Ordering::Relaxed));
    t.set_callback(|t| {
        if t.value() {
            app::foreground(0, 0, 0);
            app::background(255, 255, 255);
            LIGHT_MODE.store(true, Ordering::Relaxed);
        } else {
            app::foreground(255, 255, 255);
            let (r, g, b) = GRAY.to_rgb();
            app::background(r, g, b);
            LIGHT_MODE.store(false, Ordering::Relaxed);
        }
        app::redraw();
    });
    g.end();
    grid.insert_ext(&mut g, 3, 15, 2, 1);
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_label("Sleep duration (millis):");
    grid.insert_ext(&mut f, 6, 2, 3, 1);
    let mut g = group::Group::default().with_size(40, 30);
    let mut slider = FancyHorSlider::new(0, 0, 40, 10).center_of_parent();
    g.end();
    grid.insert_ext(&mut g, 6, 14, 4, 1);
    let val = SLEEP.load(Ordering::Relaxed);
    let mut f = frame::Frame::default()
        .with_size(0, 40)
        .with_label(&val.to_string());
    grid.insert_ext(&mut f, 7, 15, 2, 1);
    slider.set_value((val as f64 - 100.) / 1000.);
    slider.set_callback(move |s| {
        let val = (s.value() * 1000.) as u64 + 100;
        f.set_label(&val.to_string());
        SLEEP.store(val, Ordering::Relaxed);
    });
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_label("Window Transparency (%):");
    grid.insert_ext(&mut f, 9, 2, 3, 1);
    let mut g = group::Group::default().with_size(40, 30);
    let mut slider = FancyHorSlider::new(0, 0, 40, 10).center_of_parent();
    g.end();
    let mut win = unsafe {
        let mut win = window::Window::from_widget_ptr(app::first_window().unwrap().as_widget_ptr());
        win.assume_derived();
        win
    };
    let opacity = win.opacity();
    let mut f = frame::Frame::default()
        .with_size(0, 40)
        .with_label(&((opacity * 100.) as i32).to_string());
    grid.insert_ext(&mut f, 10, 15, 2, 1);
    slider.set_value(opacity);
    slider.set_callback(move |s| {
        let val = s.value();
        f.set_label(&((val * 100.) as i32).to_string());
        win.set_opacity(val);
    });
    grid.insert_ext(&mut g, 9, 14, 4, 1);
}

pub fn settings() -> group::Pack {
    let mut grp = group::Pack::default_fill().center_of_parent();
    grp.set_spacing(0);
    let mut grid = Grid::default_fill();
    grid.set_layout(20, 20);
    grid.debug(false);
    grp.end();
    fill_grid(&mut grid);
    grp
}
