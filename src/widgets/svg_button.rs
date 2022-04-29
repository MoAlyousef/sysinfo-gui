use crate::gui::colors::*;
use fltk::{enums::*, prelude::*, *};
use std::ops::{Deref, DerefMut};

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
        let mut image = image::SvgImage::from_data(svg).unwrap();
        image.scale(30, 30, true, true);
        btn.set_image(Some(image));
        Self { btn }
    }

    pub fn with_tooltip(mut self, label: &str) -> Self {
        self.btn.set_tooltip(label);
        self
    }
}

impl Deref for SvgButton {
    type Target = button::RadioButton;

    fn deref(&self) -> &Self::Target {
        &self.btn
    }
}

impl DerefMut for SvgButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.btn
    }
}
