use crate::gui::styles::colors::*;
use crate::view::MyView;
use fltk::{enums::*, prelude::*, *};
use fltk_extras::{
    button::{RoundToggle, Toggle},
    slider::FancyHorSlider,
};
use std::sync::atomic::Ordering;

pub fn settings(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut win = unsafe {
        let mut win = window::Window::from_widget_ptr(app::first_window().unwrap().as_widget_ptr());
        win.assume_derived();
        win
    };
    let mut row = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_align(Align::Left | Align::Inside)
            .with_label("Light mode:");
        let col = group::Flex::default().column();
        frame::Frame::default();
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
        frame::Frame::default();
        col.end();
        row.set_size(&col, 80);
        row.end();
        let mut row = group::Flex::default().row();
        frame::Frame::default()
            .with_align(Align::Left | Align::Inside)
            .with_label("Enable logging:");
        let col = group::Flex::default().column();
        frame::Frame::default();
        let mut t = RoundToggle::default();
        t.set_value(false);
        t.set_callback({
            move |t| {
                if t.value() {
                    eprintln!("Logging is not yet added!");
                }
                app::redraw();
            }
        });
        frame::Frame::default();
        col.end();
        row.set_size(&col, 80);
        row.end();
        let mut row = group::Flex::default().row();
        frame::Frame::default()
            .with_align(Align::Left | Align::Inside)
            .with_label("Sleep duration:");
        let col = group::Flex::default().column();
        frame::Frame::default();
        let mut slider = FancyHorSlider::default().with_size(40, 10);
        let val = view.sleep.load(Ordering::Relaxed);
        let mut f = frame::Frame::default()
            .with_size(0, 40)
            .with_label(&format!("{} ms", val));
        slider.set_value((val as f64 - 100.) / 1000.);
        let sleep = view.sleep.clone();
        slider.set_callback(move |s| {
            let val = (s.value() * 1000.) as u64 + 100;
            f.set_label(&format!("{} ms", val));
            sleep.store(val, Ordering::Relaxed);
        });
        frame::Frame::default();
        col.end();
        row.set_size(&col, 80);
        row.end();
        let mut row = group::Flex::default().row();
        frame::Frame::default()
            .with_align(Align::Left | Align::Inside)
            .with_label("Window Opacity:");
        let col = group::Flex::default().column();
        frame::Frame::default();
        let mut slider = FancyHorSlider::default().with_size(40, 20);
        let opacity = win.opacity();
        let mut f = frame::Frame::default()
            .with_size(0, 40)
            .with_label(&format!("{}%", ((opacity * 100.) as i32)));
        slider.set_value(opacity);
        slider.set_callback(move |s| {
            let val = s.value();
            f.set_label(&format!("{}%", ((val * 100.) as i32)));
            win.set_opacity(val);
        });
        frame::Frame::default();
        col.end();
        row.set_size(&col, 80);
    }
    row.end();
    None
}
