use crate::styles::colors::*;
use fltk::{enums::*, prelude::*, *};
use std::sync::{
    atomic::{AtomicBool, AtomicI32, Ordering},
    Arc,
};

#[derive(Debug, Clone)]
pub struct SvgDial {
    dial: valuator::FillDial,
    value: Arc<AtomicI32>,
    modifiable: Arc<AtomicBool>,
}

impl Default for SvgDial {
    fn default() -> Self {
        SvgDial::new(0, 0, 0, 0, "")
    }
}

impl SvgDial {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut dial = valuator::FillDial::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Top);
        dial.set_label_size(app::font_size() + 2);
        dial.set_frame(FrameType::NoBox);
        dial.set_color(Color::color_average(dial.color(), Color::Foreground, 0.9));
        dial.set_selection_color(RED);
        let value = Arc::new(AtomicI32::new(0));
        let value_c = value.clone();
        dial.draw(move |w| {
            let col = w.parent().unwrap().color().to_rgb();
            let inner = format!(
                "<svg viewBox='0 0 100 100'>
                <circle r='50' cx='50' cy='50' fill='rgb({},{},{})'/>
            </svg>",
                col.0, col.1, col.2
            );
            let mut image = image::SvgImage::from_data(&inner).unwrap();
            image.scale(w.w() * 3 / 4, w.h() * 3 / 4, false, true);
            image.draw(
                w.x() + (w.w() / 8),
                w.y() + (w.h() / 8),
                w.w() - (w.w() / 4),
                w.h() - (w.h() / 4),
            );
            let outer = format!(
                "<svg viewBox='0 0 100 100'>
            <rect
            rx='-50' 
            ry='-50' 
            width='100' 
            height='100' 
            fill='none' 
            stroke='rgb({},{},{})' 
            stroke-width='1' />\n
            </svg>",
                col.0, col.1, col.2
            );
            let mut image = image::SvgImage::from_data(&outer).unwrap();
            image.scale(w.w(), w.h(), false, true);
            image.draw(w.x(), w.y(), w.w(), w.h());
            draw::set_font(Font::Helvetica, 20);
            draw::set_draw_color(w.label_color());
            draw::draw_text2(
                &value_c.load(Ordering::Relaxed).to_string(),
                w.x(),
                w.y(),
                w.w(),
                w.h(),
                Align::Center,
            );
        });
        let modifiable = Arc::new(AtomicBool::new(true));
        let mod_c = modifiable.clone();
        let val_c = value.clone();
        dial.set_callback(move |d| {
            if mod_c.load(Ordering::Relaxed) {
                val_c.store((d.value() * 100.) as i32, Ordering::Relaxed);
            } else {
                d.set_value(val_c.load(Ordering::Relaxed) as f64 / 100.)
            }
        });
        Self {
            dial,
            value,
            modifiable,
        }
    }
    pub fn value(&self) -> i32 {
        self.value.load(Ordering::Relaxed)
    }
    pub fn set_value(&mut self, val: i32) {
        self.value.store(val, Ordering::Relaxed);
        self.dial.set_value(val as f64 / 100.);
    }
    pub fn modifiable(&mut self, val: bool) {
        self.modifiable.store(val, Ordering::Relaxed);
    }
}

fltk::widget_extends!(SvgDial, valuator::FillDial, dial);

#[derive(Debug, Clone)]
pub struct Dial {
    dial: valuator::FillDial,
    value: Arc<AtomicI32>,
    modifiable: Arc<AtomicBool>,
}

impl Default for Dial {
    fn default() -> Self {
        Dial::new(0, 0, 0, 0, "")
    }
}

impl Dial {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut dial = valuator::FillDial::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Top);
        dial.set_label_size(app::font_size() + 2);
        dial.set_frame(FrameType::NoBox);
        dial.set_color(Color::color_average(dial.color(), Color::Foreground, 0.9));
        dial.set_selection_color(RED);
        let value = Arc::new(AtomicI32::new(0));
        let value_c = value.clone();
        dial.draw(move |w| {
            draw::set_draw_color(w.parent().unwrap().color());
            draw::draw_pie(
                w.x() + (w.w() / 6),
                w.y() + (w.h() / 6),
                w.w() * 2 / 3,
                w.h() * 2 / 3,
                0.,
                360.,
            );
            draw::set_font(Font::Helvetica, 16);
            draw::set_draw_color(w.label_color());
            draw::draw_text2(
                &value_c.load(Ordering::Relaxed).to_string(),
                w.x(),
                w.y(),
                w.w(),
                w.h(),
                Align::Center,
            );
        });
        let modifiable = Arc::new(AtomicBool::new(true));
        let mod_c = modifiable.clone();
        let val_c = value.clone();
        dial.set_callback(move |d| {
            if mod_c.load(Ordering::Relaxed) {
                val_c.store((d.value() * 100.) as i32, Ordering::Relaxed);
            } else {
                d.set_value(val_c.load(Ordering::Relaxed) as f64 / 100.)
            }
        });
        Self {
            dial,
            value,
            modifiable,
        }
    }
    pub fn value(&self) -> i32 {
        self.value.load(Ordering::Relaxed)
    }
    pub fn set_value(&mut self, val: i32) {
        self.value.store(val, Ordering::Relaxed);
        self.dial.set_value(val as f64 / 100.);
    }
    pub fn modifiable(&mut self, val: bool) {
        self.modifiable.store(val, Ordering::Relaxed);
    }
}

fltk::widget_extends!(Dial, valuator::FillDial, dial);

#[derive(Debug, Clone)]
pub struct HalfDial {
    main_wid: group::Group,
    value: Arc<AtomicI32>,
    value_frame: frame::Frame,
}

impl HalfDial {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let value = AtomicI32::new(0);
        let mut main_wid = group::Group::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Top);
        main_wid.set_label_size(app::font_size() + 3);
        let mut value_frame =
            frame::Frame::new(main_wid.x(), main_wid.y() + 80, main_wid.w(), 40, "0");
        value_frame.set_label_size(app::font_size() + 12);
        main_wid.end();
        let value = Arc::new(value);
        let value_c = value.clone();
        main_wid.draw(move |w| {
            let parent = w.parent().unwrap();
            let parent_col = parent.color();
            draw::set_draw_color(Color::color_average(parent_col, Color::Foreground, 0.9));
            draw::draw_pie(w.x(), w.y(), w.w(), w.h(), 0., 180.);
            draw::set_draw_color(w.selection_color());
            let val = value_c.load(Ordering::Relaxed);
            let val = if val > 100 { 100 } else { val };
            draw::draw_pie(w.x(), w.y(), w.w(), w.h(), (100 - val) as f64 * 1.8, 180.);
            draw::set_draw_color(parent_col);
            draw::draw_pie(
                w.x() - 50 + w.w() / 2,
                w.y() - 50 + w.h() / 2,
                100,
                100,
                0.,
                360.,
            );
            w.draw_children();
        });
        Self {
            main_wid,
            value,
            value_frame,
        }
    }
    pub fn set_label_color(&mut self, col: Color) {
        self.value_frame.set_label_color(col);
        self.main_wid.set_label_color(col);
    }
    pub fn value(&self) -> i32 {
        self.value.load(Ordering::Relaxed)
    }
    pub fn set_value(&mut self, val: i32) {
        self.value.store(val, Ordering::Relaxed);
        self.value_frame.set_label(&val.to_string());
        self.main_wid.redraw();
    }
}

widget_extends!(HalfDial, group::Group, main_wid);
