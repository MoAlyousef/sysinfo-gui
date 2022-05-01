use crate::styles::colors::*;
use fltk::{enums::*, prelude::*, *};

#[derive(Clone)]
pub struct Toggle {
    btn: button::ToggleButton,
}

impl Default for Toggle {
    fn default() -> Self {
        Toggle::new(0, 0, 0, 0)
    }
}

impl Toggle {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut btn = button::ToggleButton::new(x, y, w, h, "@+6square")
            .with_align(Align::Left | Align::Inside);
        btn.set_frame(FrameType::FlatBox);
        btn.set_down_frame(FrameType::FlatBox);
        btn.set_color(RED);
        btn.set_selection_color(GREEN);
        btn.set_label_color(Color::White);
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
        Self { btn }
    }
}

fltk::widget_extends!(Toggle, button::ToggleButton, btn);

#[derive(Clone)]
pub struct RoundToggle {
    btn: button::ToggleButton,
}

impl Default for RoundToggle {
    fn default() -> Self {
        RoundToggle::new(0, 0, 0, 0)
    }
}

impl RoundToggle {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut btn = button::ToggleButton::new(x, y, w, h, None);
        btn.set_frame(FrameType::NoBox);
        btn.set_down_frame(FrameType::NoBox);
        btn.set_selection_color(GREEN);
        btn.set_color(RED);
        btn.set_label_color(Color::White);
        btn.clear_visible_focus();
        btn.draw(|b| {
            let col = if b.value() {
                b.selection_color().to_rgb()
            } else {
                b.color().to_rgb()
            };
            let svg = format!(
                "<svg viewBox='0 0 {} {}'>
                <rect x='1%' y='1%' rx='15' width='98%' height='98%' fill='rgb({},{},{})'/>
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
            let col = b.label_color().to_rgb();
            let svg = format!(
                "<svg viewBox='0 0 100 100'>
            <circle cx='50' cy='50' r='50' fill='rgb({},{},{})'/>
            </svg>",
                col.0, col.1, col.2
            );
            let mut image = image::SvgImage::from_data(&svg).unwrap();
            image.scale(30, 30, false, true);
            if b.value() {
                image.draw(b.x() + b.w() + 5 - 30, b.y() + ((b.h() - 30) / 2), 30, 30);
            } else {
                image.draw(b.x() - 5, b.y() + ((b.h() - 30) / 2), 30, 30);
            }
        });
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
        Self { btn }
    }
}

fltk::widget_extends!(RoundToggle, button::ToggleButton, btn);

#[derive(Clone)]
pub struct HollowRoundToggle {
    btn: button::ToggleButton,
}

impl Default for HollowRoundToggle {
    fn default() -> Self {
        HollowRoundToggle::new(0, 0, 0, 0)
    }
}

impl HollowRoundToggle {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut btn = button::ToggleButton::new(x, y, w, h, None);
        btn.set_frame(FrameType::NoBox);
        btn.set_down_frame(FrameType::NoBox);
        btn.set_selection_color(GREEN);
        btn.set_color(RED);
        btn.set_label_color(Color::White);
        btn.clear_visible_focus();
        btn.draw(|b| {
            let col = if b.value() {
                b.selection_color().to_rgb()
            } else {
                b.color().to_rgb()
            };
            let svg = format!(
                "<svg viewBox='0 0 {} {}'>
            <rect x='2%' y='2%' rx='15' width='96%' height='96%' fill='none' stroke='rgb({},{},{})'/>
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
            image.scale(30, 30, false, true);
            if b.value() {
                image.draw(b.x() + b.w() - 3 - 30, b.y() + ((b.h() - 30)/2), 30, 30);
            } else {
                image.draw(b.x() + 3, b.y() + ((b.h() - 30)/2), 30, 30);
            }
        });
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
        Self { btn }
    }
}

fltk::widget_extends!(HollowRoundToggle, button::ToggleButton, btn);
