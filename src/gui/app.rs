use super::styles::colors::*;
use super::styles::svgs::*;
use super::widgets::*;
use super::{message::Message, View};
use fltk::{enums::*, prelude::*, *};

pub struct App {
    a: app::App,
    r: app::Receiver<Message>,
    scroll: group::Scroll,
}

impl App {
    pub fn new() -> Self {
        std::panic::set_hook(Box::new(|info| {
            if let Some(s) = info.payload().downcast_ref::<&str>() {
                // we shamefully use those to end spawned threads
                if !s.contains("self.was_deleted") {
                    fltk::dialog::message_default(s);
                }
            } else if let Some(s) = info.payload().downcast_ref::<String>() {
                if !s.contains("self.was_deleted") {
                    fltk::dialog::message_default(s);
                }
            } else {
                fltk::dialog::message_default(&format!("{:?}", info));
            }
        }));
        let a = app::App::default();
        let (r, g, b) = GRAY.to_rgb();
        app::background(r, g, b);
        app::foreground(255, 255, 255);
        app::set_frame_type2(FrameType::UpBox, FrameType::FlatBox);
        let (r, g, b) = SEL_BLUE.to_rgb();
        app::set_selection_color(r, g, b);
        misc::Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xF0));
        app::set_font_size(18);
        let temp = std::env::temp_dir().join("Roboto-Medium.ttf");
        if !temp.exists() {
            let bytes = include_bytes!("../../Roboto-Medium.ttf");
            std::fs::write(&temp, bytes).ok();
        }
        if let Ok(f) = Font::load_font(temp) {
            Font::set_font(Font::Helvetica, &f);
        }
        let (s, r) = app::channel();
        let mut win = window::Window::default()
            .with_size(800, 600)
            .with_label("sysinfo-gui");
        win.set_xclass("sysinfo");
        let mut grp = group::Group::default().with_size(60, 600);
        grp.set_frame(FrameType::FlatBox);
        grp.set_color(BLUE);
        let mut col = group::Pack::default()
            .with_size(40, 600)
            .center_of_parent()
            .with_type(group::PackType::Vertical);
        col.set_spacing(10);
        SvgButton::new("<svg></svg>");
        SvgButton::new(GENERAL)
            .with_tooltip("Home")
            .toggled(true)
            .emit(s, Message::General);
        SvgButton::new(LIST)
            .with_tooltip("Processes")
            .emit(s, Message::Procs);
        SvgButton::new(PROC)
            .with_tooltip("Processors info")
            .emit(s, Message::Proc);
        SvgButton::new(MEMORY)
            .with_tooltip("Memory info")
            .emit(s, Message::Memory);
        SvgButton::new(DISKS)
            .with_tooltip("Disks info")
            .emit(s, Message::Disks);
        SvgButton::new(NET)
            .with_tooltip("Network info")
            .emit(s, Message::Net);
        SvgButton::new(WRENCH)
            .with_tooltip("Settings")
            .emit(s, Message::Settings);
        col.end();
        grp.end();
        let mut grp = group::Group::new(60, 0, 800 - 50, 50, "\tSysinfo")
            .with_align(Align::Left | Align::Inside);
        grp.set_label_color(Color::White);
        grp.set_label_size(app::font_size() + 5);
        grp.set_frame(FrameType::FlatBox);
        grp.set_color(BLUE);
        grp.end();
        let mut scroll = group::Scroll::new(60, 50, 800 - 50, 600 - 50, None);
        scroll.set_color(win.color());
        scroll.set_scrollbar_size(-1);
        let mut scrollbar = scroll.scrollbar();
        scrollbar.set_callback({
            let mut old_cb = scrollbar.callback();
            move |s| {
                if let Some(cb) = old_cb.as_mut() {
                    (*cb)();
                }
                s.parent().unwrap().redraw();
            }
        });
        scroll.end();
        win.end();
        win.show();
        win.set_callback(|w| {
            if app::event() == Event::Close {
                w.hide();
            }
        });
        Self { a, r, scroll }
    }
    pub fn run(mut self, view: impl View + 'static) {
        self.scroll.begin();
        let cb = view.view(Message::General);
        Self::dispatch(cb, view.sleep_duration());
        self.scroll.end();
        while self.a.wait() {
            if let Some(msg) = self.r.recv() {
                self.scroll.clear();
                self.scroll.begin();
                let cb = view.view(msg);
                Self::dispatch(cb, view.sleep_duration());
                self.scroll.end();
                app::redraw();
            }
        }
    }
    fn dispatch(cb: Option<Box<dyn FnMut() + Send>>, sleep: u64) {
        if let Some(mut cb) = cb {
            std::thread::spawn({
                move || loop {
                    cb();
                    std::thread::sleep(std::time::Duration::from_millis(sleep));
                }
            });
        }
    }
}
