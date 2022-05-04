use crate::gui::styles::colors::*;
use fltk::{enums::*, prelude::*, *};

#[derive(Debug, Clone)]
pub struct FancyHorSlider {
    p: group::Pack,
    s: valuator::Slider,
}

impl Default for FancyHorSlider {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, "")
    }
}

impl FancyHorSlider {
    pub fn new(x: i32, y: i32, width: i32, height: i32, label: &str) -> Self {
        let p = group::Pack::new(x, y, width, height, None).with_label(label);
        frame::Frame::default().with_size(width, 5);
        let mut s = valuator::Slider::new(x, y, width, 10, None);
        s.set_type(valuator::SliderType::Horizontal);
        s.set_frame(FrameType::RFlatBox);
        s.set_color(SLIDER_PURPLE);
        s.draw(|s| {
            draw::set_draw_color(SEL_BLUE);
            draw::draw_pie(
                s.x() - 10 + (s.w() as f64 * s.value()) as i32,
                s.y() - 10,
                30,
                30,
                0.,
                360.,
            );
        });
        s.set_callback(|_| app::redraw());
        p.end();
        Self { p, s }
    }

    pub fn set_callback<F: 'static + FnMut(&mut Self)>(&mut self, mut cb: F) {
        let mut s = self.clone();
        self.s.set_callback(move |_| {
            cb(&mut s);
            app::redraw();
        });
    }
    pub fn value(&self) -> f64 {
        self.s.value()
    }
    pub fn set_value(&mut self, val: f64) {
        self.s.set_value(val)
    }
}

fltk::widget_extends!(FancyHorSlider, group::Pack, p);
