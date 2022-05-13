use crate::gui::styles::colors::*;
use crate::gui::widgets::{FancyHorSlider, Toggle};
use crate::view::MyView;
use fltk::{enums::*, prelude::*, *};
use fltk_grid::Grid;
use std::sync::atomic::Ordering;

fn fill_grid(grid: &mut Grid, view: &MyView) {
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_label("Light mode:");
    grid.insert_ext(&mut f, 3, 2, 3, 1);
    let mut t = Toggle::default();
    t.set_value(view.light_mode.load(Ordering::Relaxed));
    let light_mode = view.light_mode.clone();
    t.set_callback(move |t| {
        if t.value() {
            app::foreground(50, 50, 50);
            app::background(255, 255, 255);
            light_mode.store(true, Ordering::Relaxed);
        } else {
            app::foreground(255, 255, 255);
            let (r, g, b) = GRAY.to_rgb();
            app::background(r, g, b);
            light_mode.store(false, Ordering::Relaxed);
        }
        app::redraw();
    });
    grid.insert_ext(&mut *t, 3, 15, 2, 1);
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_label("Sleep duration:");
    grid.insert_ext(&mut f, 6, 2, 3, 1);
    let mut slider = FancyHorSlider::default()
        .with_size(40, 10)
        .center_of_parent();
    grid.insert_ext(&mut *slider, 6, 14, 4, 1);
    let val = view.sleep.load(Ordering::Relaxed);
    let mut f = frame::Frame::default()
        .with_size(0, 40)
        .with_label(&format!("{} ms", val));
    grid.insert_ext(&mut f, 7, 15, 2, 1);
    slider.set_value((val as f64 - 100.) / 1000.);
    let sleep = view.sleep.clone();
    slider.set_callback(move |s| {
        let val = (s.value() * 1000.) as u64 + 100;
        f.set_label(&format!("{} ms", val));
        sleep.store(val, Ordering::Relaxed);
    });
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_label("Window Opacity:");
    grid.insert_ext(&mut f, 9, 2, 3, 1);
    let mut slider = FancyHorSlider::default()
        .with_size(40, 20)
        .center_of_parent();
    let mut win = unsafe {
        let mut win = window::Window::from_widget_ptr(app::first_window().unwrap().as_widget_ptr());
        win.assume_derived();
        win
    };
    let opacity = win.opacity();
    let mut f = frame::Frame::default()
        .with_size(0, 40)
        .with_label(&format!("{}%", ((opacity * 100.) as i32)));
    grid.insert_ext(&mut f, 10, 15, 2, 1);
    slider.set_value(opacity);
    slider.set_callback(move |s| {
        let val = s.value();
        f.set_label(&format!("{}%", ((val * 100.) as i32)));
        win.set_opacity(val);
    });
    grid.insert_ext(&mut *slider, 9, 14, 4, 1);
}

pub fn settings(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut grp = group::Pack::default_fill().center_of_parent();
    grp.set_spacing(0);
    let mut grid = Grid::default_fill();
    grid.set_layout(20, 20);
    // grid.debug(true);
    grp.end();
    fill_grid(&mut grid, view);
    None
}
