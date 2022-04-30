use crate::styles::colors::*;
use fltk::{enums::*, prelude::*, *};
use std::ops::{Deref, DerefMut};

pub struct Card {
    grp: group::Group,
}

impl Card {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Card {
        let mut grp = group::Group::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Top | Align::Left);
        grp.set_label_size(16);
        grp.set_color(GRAY.inactive());
        grp.set_label_color(Color::White);
        grp.draw(|g| {
            let col = g.color();
            let (r1, g1, b1) = col.to_rgb();
            let (r2, g2, b2) = col.darker().to_rgb();
            let svg = format!(
                "<svg viewBox='0 0 {} {}'>
            <defs>
            <linearGradient id='grad1' x1='0%' y1='0%' x2='0%' y2='100%'>
            <stop offset='0%' style='stop-color:rgb({},{},{});stop-opacity:1' />
            <stop offset='100%' style='stop-color:rgb({},{},{});stop-opacity:1' />
            </linearGradient>
            </defs>
            <rect width='98%' height='98%' rx='10' fill='url(#grad1)' />
            </svg>",
                g.w(),
                g.h(),
                r1,
                g1,
                b1,
                r2,
                g2,
                b2
            );
            let mut image = image::SvgImage::from_data(&svg).unwrap();
            image.scale(g.w(), g.h(), false, true);
            image.draw(g.x(), g.y(), g.w(), g.h());
            g.draw_children();
        });
        grp.end();
        Self { grp }
    }
}

impl Deref for Card {
    type Target = group::Group;

    fn deref(&self) -> &Self::Target {
        &self.grp
    }
}

impl DerefMut for Card {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grp
    }
}
