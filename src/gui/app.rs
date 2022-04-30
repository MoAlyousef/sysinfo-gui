use super::{message::Message, View};
use crate::styles::colors::*;
use crate::styles::svgs::*;
use crate::widgets::*;
use fltk::{enums::*, prelude::*, *};

pub struct App {
    a: app::App,
    r: app::Receiver<Message>,
    scroll: group::Scroll,
    view: Box<dyn View>,
}

impl App {
    pub fn new(view: impl View + 'static) -> Self {
        let a = app::App::default();
        app::background(0x32, 0x32, 0x32);
        misc::Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xF0));
        let (s, r) = app::channel();
        let mut win = window::Window::default().with_size(800, 600);
        let mut grp = group::Group::new(0, 0, 60, 600, None);
        grp.set_frame(FrameType::FlatBox);
        grp.set_color(BLUE);
        let mut col = group::Pack::default()
            .with_size(40, 600)
            .center_of_parent()
            .with_type(group::PackType::Vertical);
        col.set_spacing(10);
        SvgButton::new(LIST)
            .with_tooltip("Home")
            .toggled(true)
            .emit(s, Message::General);
        SvgButton::new(PROC)
            .with_tooltip("Processors info")
            .emit(s, Message::Proc);
        SvgButton::new(MEMORY)
            .with_tooltip("Memory info")
            .emit(s, Message::Memory);
        SvgButton::new(DISKS)
            .with_tooltip("Disks info")
            .emit(s, Message::Disks);
        SvgButton::new(PROCS)
            .with_tooltip("Processes")
            .emit(s, Message::Procs);
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
        grp.set_label_size(20);
        grp.set_frame(FrameType::FlatBox);
        grp.set_color(BLUE);
        grp.end();
        let mut scroll = group::Scroll::new(60, 50, 800 - 50, 600 - 50, None);
        scroll.set_color(win.color());
        scroll.set_scrollbar_size(-1);

        view.view(Message::General);

        scroll.end();
        win.end();
        win.show();
        win.set_callback(|w| {
            if app::event() == Event::Close {
                w.hide();
            }
        });
        Self {
            a,
            r,
            scroll,
            view: Box::new(view),
        }
    }
    pub fn run(mut self) {
        while self.a.wait() {
            if let Some(msg) = self.r.recv() {
                self.scroll.clear();
                self.scroll.begin();

                self.view.view(msg);

                self.scroll.end();
                app::redraw();
            }
        }
    }
}
