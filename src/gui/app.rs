use super::styles::colors::*;
use super::styles::svgs::*;
use super::widgets::*;
use super::{message::Message, View};
use fltk::{enums::*, prelude::*, *};

const ICON: &[u8] = include_bytes!("../../assets/icon.png");

pub struct App {
    a: app::App,
    r: app::Receiver<Message>,
    main_view: group::Flex,
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
        app::set_font_size(16);
        misc::Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xF0));
        misc::Tooltip::set_font_size(app::font_size() - 4);
        let temp = std::env::temp_dir().join("Roboto-Medium.ttf");
        if !temp.exists() {
            let bytes = include_bytes!("../../assets/Roboto-Medium.ttf");
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
        win.set_icon(Some(image::PngImage::from_data(ICON).unwrap()));
        let mut main_col = group::Column::default_fill();
        main_col.set_pad(0);
        let mut grp = group::Group::default()
            .with_label("\tSysinfo")
            .with_align(Align::Left | Align::Inside);
        main_col.set_size(&grp, 50);
        grp.set_label_color(Color::White);
        grp.set_label_size(app::font_size() + 5);
        grp.set_frame(FrameType::FlatBox);
        grp.set_color(BLUE);
        grp.end();
        let mut main_row = group::Row::default_fill();
        main_row.set_pad(0);
        let mut col = group::Column::default();
        main_row.set_size(&*col, 60);
        col.set_frame(FrameType::FlatBox);
        col.set_color(BLUE);
        col.set_pad(10);
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
        SvgButton::new(ABOUT)
            .with_tooltip("About")
            .emit(s, Message::Info);
        col.end();
        let mut main_view = group::Flex::default().column();
        main_view.set_margin(50);
        main_view.end();
        main_row.end();
        main_col.end();
        win.end();
        win.resizable(&main_view);
        win.size_range(800, 600, 0, 0);
        win.show();
        win.set_callback(|w| {
            if app::event() == Event::Close {
                w.hide();
            }
        });
        Self { a, r, main_view }
    }
    pub fn run(mut self, view: impl View + 'static) {
        self.main_view.begin();
        let cb = view.view(Message::General);
        Self::dispatch(cb, view.sleep_duration());
        self.main_view.end();
        while self.a.wait() {
            if let Some(msg) = self.r.recv() {
                self.main_view.clear();
                self.main_view.begin();
                let cb = view.view(msg);
                Self::dispatch(cb, view.sleep_duration());
                self.main_view.end();
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
