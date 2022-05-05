use crate::gui::styles::colors::*;
use fltk::{enums::*, prelude::*, *};

#[derive(Clone)]
pub struct Toggle {
    p: group::Pack,
    btn: button::ToggleButton,
}

impl Default for Toggle {
    fn default() -> Self {
        Toggle::new(0, 0, 0, 0, "")
    }
}

impl Toggle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let p = group::Pack::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Left);
        frame::Frame::default().with_size(w, 7);
        let mut btn = button::ToggleButton::new(x, y, w, 14, "@+6square")
            .with_align(Align::Left | Align::Inside);
        btn.set_frame(FrameType::FlatBox);
        btn.set_down_frame(FrameType::FlatBox);
        btn.set_label_color(Color::White);
        btn.set_color(RED);
        btn.set_selection_color(GREEN);
        btn.clear_visible_focus();
        btn.handle(|b, ev| match ev {
            Event::Push => {
                if b.value() {
                    b.set_align(Align::Left | Align::Inside);
                } else {
                    b.set_align(Align::Right | Align::Inside);
                }
                app::redraw();
                true
            }
            _ => false,
        });
        p.end();
        Self { btn, p }
    }
    pub fn set_value(&mut self, val: bool) {
        self.btn.set_value(val);
        if self.btn.value() {
            self.btn.set_align(Align::Right | Align::Inside);
        } else {
            self.btn.set_align(Align::Left | Align::Inside);
        }
        app::redraw();
    }
    pub fn value(&self) -> bool {
        self.btn.value()
    }
    pub fn set_callback<F: 'static + FnMut(&mut Self)>(&mut self, mut cb: F) {
        let mut s = self.clone();
        self.btn.set_callback(move |_| {
            cb(&mut s);
            app::redraw();
        });
    }
}

fltk::widget_extends!(Toggle, group::Pack, p);

#[derive(Clone)]
pub struct RoundToggle {
    p: group::Pack,
    btn: button::ToggleButton,
}

impl Default for RoundToggle {
    fn default() -> Self {
        RoundToggle::new(0, 0, 0, 0, "")
    }
}

impl RoundToggle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let p = group::Pack::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Left);
        let mut btn = button::ToggleButton::new(x, y, w, 30, "@+6circle")
            .with_align(Align::Left | Align::Inside);
        btn.set_frame(FrameType::RFlatBox);
        btn.set_down_frame(FrameType::RFlatBox);
        btn.set_label_color(Color::White);
        btn.set_color(RED);
        btn.set_selection_color(GREEN);
        btn.clear_visible_focus();
        btn.handle(|b, ev| match ev {
            Event::Push => {
                if b.value() {
                    b.set_align(Align::Left | Align::Inside);
                } else {
                    b.set_align(Align::Right | Align::Inside);
                }
                app::redraw();
                true
            }
            _ => false,
        });
        p.end();
        Self { btn, p }
    }
    pub fn set_value(&mut self, val: bool) {
        self.btn.set_value(val);
        if self.btn.value() {
            self.btn.set_align(Align::Right | Align::Inside);
        } else {
            self.btn.set_align(Align::Left | Align::Inside);
        }
        app::redraw();
    }
    pub fn value(&self) -> bool {
        self.btn.value()
    }
    pub fn set_callback<F: 'static + FnMut(&mut Self)>(&mut self, mut cb: F) {
        let mut s = self.clone();
        self.btn.set_callback(move |_| {
            cb(&mut s);
            app::redraw();
        });
    }
}

fltk::widget_extends!(RoundToggle, group::Pack, p);

#[derive(Clone)]
pub struct HollowRoundToggle {
    btn: button::ToggleButton,
}

impl Default for HollowRoundToggle {
    fn default() -> Self {
        HollowRoundToggle::new(0, 0, 0, 0, "")
    }
}

impl HollowRoundToggle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut btn = button::ToggleButton::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Left);
        btn.set_frame(FrameType::NoBox);
        btn.set_down_frame(FrameType::NoBox);
        btn.set_selection_color(GREEN);
        btn.set_color(RED);
        btn.clear_visible_focus();
        btn.draw(|b| {
            let col = if b.value() {
                b.selection_color().to_rgb()
            } else {
                b.color().to_rgb()
            };
            let svg = format!(
                "<svg viewBox='0 0 {} {}'>
            <rect x='1%' y='1%' rx='15' width='98%' height='98%' fill='none' stroke='rgb({},{},{})'/>
            </svg>",
                b.w(),
                b.h(),
                col.0,
                col.1,
                col.2
            );
            let mut image = image::SvgImage::from_data(&svg).unwrap();
            image.scale(b.w(), b.h(), false, true);
            image.draw(b.x(), b.y(), b.w(), b.h());
            let svg = format!(
                "<svg viewBox='0 0 100 100'>
            <circle cx='50' cy='50' r='50' fill='rgb({},{},{})'/>
            </svg>",
                col.0, col.1, col.2
            );
            let mut image = image::SvgImage::from_data(&svg).unwrap();
            image.scale(18, 18, false, true);
            if b.value() {
                image.draw(b.x() + b.w() - 3 - 18, b.y() + ((b.h() - 18)/2), 18, 18);
            } else {
                image.draw(b.x() + 3, b.y() + ((b.h() - 18)/2), 18, 18);
            }
        });
        Self { btn }
    }
}

fltk::widget_extends!(HollowRoundToggle, button::ToggleButton, btn);
