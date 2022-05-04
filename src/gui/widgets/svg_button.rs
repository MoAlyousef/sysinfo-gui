use crate::gui::styles::colors::*;
use fltk::{enums::*, prelude::*, *};

pub struct SvgButton {
    btn: button::RadioButton,
}

impl SvgButton {
    pub fn new(svg: &str) -> SvgButton {
        let mut btn = button::RadioButton::new(0, 0, 50, 50, None);
        btn.set_frame(FrameType::FlatBox);
        btn.set_down_frame(FrameType::FlatBox);
        btn.set_color(BLUE);
        btn.set_selection_color(SEL_BLUE);
        btn.clear_visible_focus();
        if let Ok(mut image) = image::SvgImage::from_data(svg) {
            image.scale(30, 30, true, true);
            btn.set_image(Some(image));
        }
        Self { btn }
    }

    pub fn with_tooltip(mut self, label: &str) -> Self {
        self.btn.set_tooltip(label);
        self
    }

    pub fn toggled(mut self, flag: bool) -> Self {
        self.btn.toggle(flag);
        self
    }
}

fltk::widget_extends!(SvgButton, button::RadioButton, btn);
