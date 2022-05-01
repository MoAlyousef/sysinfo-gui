// use crate::styles::colors::*;
use crate::styles::colors::*;
use fltk::{enums::*, prelude::*, *};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct FancyHorSlider {
    s: valuator::Slider,
}

impl FancyHorSlider {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut s = valuator::Slider::new(x, y, width, height, None);
        s.set_type(valuator::SliderType::Horizontal);
        s.set_frame(FrameType::RFlatBox);
        s.set_color(SLIDER_PURPLE);
        s.draw(|s| {
            draw::set_draw_color(Color::Blue);
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
        Self { s }
    }

    pub fn set_callback<F: 'static + FnMut(&mut Self)>(&mut self, mut cb: F) {
        let mut s = self.clone();
        self.s.set_callback(move |_| {
            cb(&mut s);
            app::redraw();
        });
    }
}

impl Deref for FancyHorSlider {
    type Target = valuator::Slider;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl DerefMut for FancyHorSlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}
