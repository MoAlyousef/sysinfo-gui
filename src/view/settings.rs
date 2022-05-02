use super::SLEEP;
use crate::widgets::{FancyHorSlider, HollowRoundToggle, RoundToggle, Toggle};
use fltk::{enums::*, prelude::*, *};
use std::sync::atomic::Ordering;

pub fn settings() -> group::Pack {
    let mut grp = group::Pack::default()
        .with_size(160, 100)
        .center_of_parent();
    grp.set_spacing(0);
    RoundToggle::new(300, 300, 60, 30);
    frame::Frame::default().with_size(0, 40);
    HollowRoundToggle::new(300, 300, 60, 34);
    frame::Frame::default().with_size(0, 40);
    Toggle::new(300, 300, 60, 15);
    frame::Frame::default().with_size(0, 40);
    let mut slider = FancyHorSlider::new(200, 200, 200, 10);
    let val = SLEEP.load(Ordering::Relaxed);
    let mut f = frame::Frame::default()
        .with_size(0, 40)
        .with_label(&val.to_string());
    f.set_label_color(Color::White);
    slider.set_value(val as f64 / 1000.);
    slider.set_callback(move |s| {
        let val = (s.value() * 1000.) as u64 + 100;
        f.set_label(&val.to_string());
        SLEEP.store(val, Ordering::Relaxed);
    });
    let mut slider = FancyHorSlider::new(200, 200, 200, 10);
    let mut win = unsafe {
        let mut win = window::Window::from_widget_ptr(app::first_window().unwrap().as_widget_ptr());
        win.assume_derived();
        win
    };
    let opacity = win.opacity();
    let mut f = frame::Frame::default()
        .with_size(0, 40)
        .with_label(&((opacity * 100.) as i32).to_string());
    f.set_label_color(Color::White);
    slider.set_value(opacity);
    slider.set_callback(move |s| {
        let val = s.value();
        f.set_label(&((val* 100.) as i32).to_string());
        win.set_opacity(val);
    });
    grp.end();
    grp
}
