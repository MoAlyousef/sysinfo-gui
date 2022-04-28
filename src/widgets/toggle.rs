use fltk::{enums::*, prelude::*, *};

#[derive(Clone)]
pub struct Toggle {
    grp: group::Group,
    frm: frame::Frame,
    btn: button::ToggleButton,
}

impl Default for Toggle {
    fn default() -> Self {
        Toggle::new(0, 0, 0, 0, "")
    }
}

impl Toggle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut grp = group::Group::new(x, y, w, h, None);
        let frm = frame::Frame::new(x, y, w - (w / 3), h, None).with_label(label);
        let mut btn =
            button::ToggleButton::new(x + (w * 2 / 3), y + (h / 3), w / 3, h / 3, "@+6square")
                .with_align(Align::Left | Align::Inside);
        btn.set_frame(FrameType::FlatBox);
        btn.set_down_frame(FrameType::FlatBox);
        btn.set_color(Color::Green);
        btn.set_selection_color(Color::Red);
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
        grp.end();
        grp.resize_callback({
            let mut frm = frm.clone();
            let mut btn = btn.clone();
            move |_, x, y, w, h| {
                frm.resize(x, y, w - (w / 3), h);
                btn.resize(x + (w * 2 / 3), y + (h / 3), w / 3, h / 3);
            }
        });
        Self { grp, frm, btn }
    }

    pub fn set_callback<F: 'static + FnMut(&mut Self)>(&mut self, mut cb: F) {
        let mut s = self.clone();
        self.btn.set_callback(move |_| cb(&mut s));
    }
    pub fn set_label(&mut self, label: &str) {
        self.frm.set_label(label);
    }
    pub fn set_label_color(&mut self, col: Color) {
        self.frm.set_label_color(col);
    }
    pub fn set_toggle_color(&mut self, col: Color) {
        self.btn.set_label_color(col);
    }
    pub fn set_on_color(&mut self, col: Color) {
        self.btn.set_color(col);
    }
    pub fn set_off_color(&mut self, col: Color) {
        self.btn.set_selection_color(col);
    }
}

fltk::widget_extends!(Toggle, group::Group, grp);

#[derive(Clone)]
pub struct RoundToggle {
    grp: group::Group,
    frm: frame::Frame,
    btn: button::ToggleButton,
}

impl Default for RoundToggle {
    fn default() -> Self {
        RoundToggle::new(0, 0, 0, 0, "")
    }
}

impl RoundToggle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut grp = group::Group::new(x, y, w, h, None);
        let frm = frame::Frame::new(x, y, w - (w / 3), h, None).with_label(label);
        let mut btn = button::ToggleButton::new(x + (w * 2 / 3), y + (h / 3), w / 3, h / 3, None);
        btn.set_frame(FrameType::NoBox);
        btn.set_down_frame(FrameType::NoBox);
        btn.set_color(Color::Green);
        btn.set_selection_color(Color::Red);
        btn.set_label_color(Color::White);
        btn.clear_visible_focus();
        btn.draw(|b| {
            let svg = if b.value() {
                let col = b.color().to_rgb();
                let svg = format!(
                    "<svg viewBox='0 0 {} {}'>
                <rect x='2%' y='2%' rx='10' width='96%' height='96%' fill='rgb({},{},{})'/>
                </svg>",
                    b.w(),
                    b.h(),
                    col.0,
                    col.1,
                    col.2
                );
                svg
            } else {
                let col = b.selection_color().to_rgb();
                let svg = format!(
                    "<svg viewBox='0 0 {} {}'>
                <rect x='2%' y='2%' rx='10' width='96%' height='96%' fill='rgb({},{},{})'/>
                </svg>",
                    b.w(),
                    b.h(),
                    col.0,
                    col.1,
                    col.2
                );
                svg
            };
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
                image.draw(b.x() + b.w() + 5 - 30, b.y() - (b.h() / 3), 30, 30);
            } else {
                image.draw(b.x() - 5, b.y() - (b.h() / 3), 30, 30);
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
        grp.end();
        grp.resize_callback({
            let mut frm = frm.clone();
            let mut btn = btn.clone();
            move |_, x, y, w, h| {
                frm.resize(x, y, w - (w / 3), h);
                btn.resize(x + (w * 2 / 3), y + (h / 3), w / 3, h / 3);
            }
        });
        Self { grp, frm, btn }
    }

    pub fn set_callback<F: 'static + FnMut(&mut Self)>(&mut self, mut cb: F) {
        let mut s = self.clone();
        self.btn.set_callback(move |_| cb(&mut s));
    }
    pub fn set_label(&mut self, label: &str) {
        self.frm.set_label(label);
    }
    pub fn set_label_color(&mut self, col: Color) {
        self.frm.set_label_color(col);
    }
    pub fn set_toggle_color(&mut self, col: Color) {
        self.btn.set_label_color(col);
    }
    pub fn set_on_color(&mut self, col: Color) {
        self.btn.set_color(col);
    }
    pub fn set_off_color(&mut self, col: Color) {
        self.btn.set_selection_color(col);
    }
    pub fn value(&self) -> bool {
        self.btn.value()
    }
}

fltk::widget_extends!(RoundToggle, group::Group, grp);
