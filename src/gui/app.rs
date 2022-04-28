use crate::gui::svgs::*;
use crate::widgets::*;
use fltk::{enums::*, prelude::*, *};

use super::colors::*;
use super::message::Message;

pub struct App {
    a: app::App,
    win: window::Window,
    r: app::Receiver<Message>,
    scroll: group::Scroll,
}

impl App {
    pub fn new() -> Self {
        let a = app::App::default();
        app::background(0x32, 0x32, 0x32);
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
        SvgButton::new(LIST).emit(s, Message::General);
        SvgButton::new(DISKS).emit(s, Message::Disks);
        SvgButton::new(PROC).emit(s, Message::Proc);
        SvgButton::new(MEMORY).emit(s, Message::Memory);
        SvgButton::new(NET).emit(s, Message::Net);
        SvgButton::new(THERM).emit(s, Message::Therm);
        SvgButton::new(WRENCH).emit(s, Message::Settings);
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
        super::view::view(Message::General);
        scroll.end();
        win.end();
        win.show();
        win.emit(s, Message::Quit);
        Self {
            a,
            win,
            r,
            scroll,
        }
    }
    pub fn run(mut self) {
        while self.a.wait() {
            if let Some(msg) = self.r.recv() {
                match msg {
                    Message::Quit => {
                        if app::event() == Event::Close {
                            self.win.hide();
                        }
                    }
                    Message::General
                    | Message::Disks
                    | Message::Therm
                    | Message::Proc
                    | Message::Memory
                    | Message::Net => {
                        self.scroll.clear();
                        self.scroll.begin();
                        super::view::view(msg);
                        self.scroll.end();
                        app::redraw();
                    }
                    _ => (),
                }
            }
        }
    }
}
