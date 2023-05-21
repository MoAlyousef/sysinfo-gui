use super::{MyView, SortOrder};
use crate::gui::styles;
use crate::gui::styles::colors::*;
use fltk::{app::MouseButton, enums::*, prelude::*, *};
use parking_lot::Mutex;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use sysinfo::ProcessExt;
use sysinfo::System;
use sysinfo::SystemExt;

struct ProcToggle {
    b: button::RadioButton,
}

impl ProcToggle {
    pub fn new(label: &str, ord: Arc<Mutex<SortOrder>>) -> Self {
        let mut b = button::RadioButton::default()
            .with_size(70, 0)
            .with_label(label)
            .with_align(Align::Left | Align::Inside);
        b.set_down_frame(FrameType::FlatBox);
        b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
        b.clear_visible_focus();
        b.set_label_size(app::font_size() - 2);
        b.draw(move |b| {
            if b.value() {
                let mut image = if (*ord.lock() as i32) < 5 {
                    image::SvgImage::from_data(styles::svgs::DESC).unwrap()
                } else {
                    image::SvgImage::from_data(styles::svgs::ASC).unwrap()
                };
                image.scale(15, 15, true, true);
                image.draw(b.x() + (b.w() * 2 / 3) + 5, b.y() + 10, b.w() / 3, b.h());
            }
        });
        b.set_frame(FrameType::FlatBox);
        Self { b }
    }
}

fltk::widget_extends!(ProcToggle, button::RadioButton, b);

struct Proc {
    pub pid: sysinfo::Pid,
    pub memory: u64,
    pub virt: u64,
    pub cpu: f32,
    pub exe: String,
    // pub total_written_bytes: u64,
    // pub written_bytes: u64,
    // pub total_read_bytes: u64,
    // pub read_bytes: u64,
}

impl Proc {
    pub fn new(pid: &sysinfo::Pid, proc: &sysinfo::Process) -> Self {
        Self {
            pid: *pid,
            memory: proc.memory(),
            virt: proc.virtual_memory(),
            cpu: proc.cpu_usage(),
            exe: proc.name().to_string(),
            // total_written_bytes: 0,
            // written_bytes: 0,
            // total_read_bytes: 0,
            // read_bytes: 0,
        }
    }
    pub fn fmt(&self, light: bool) -> String {
        if !light {
            format!(
                "@C255 {}\t@C255 {:.01}\t@C255 {:.01}\t@C255 {:.01}\t@C255{}",
                self.pid,
                self.memory as f64 / 2_f64.powf(20.),
                self.virt as f64 / 2_f64.powf(20.),
                self.cpu,
                self.exe
            )
        } else {
            format!(
                " {}\t {:.01}\t {:.01}\t {:.01}\t{}",
                self.pid,
                self.memory as f64 / 2_f64.powf(20.),
                self.virt as f64 / 2_f64.powf(20.),
                self.cpu,
                self.exe
            )
        }
    }
}

pub fn procs(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut ord = view.ordering.lock();
    *ord = SortOrder::Pid;
    drop(ord);
    let mut sys = view.system.lock();
    sys.refresh_processes();
    let hpack = group::Pack::default().with_type(group::PackType::Horizontal);
    let mut parent = group::Flex::from_dyn_widget(&hpack.parent().unwrap()).unwrap();
    parent.set_size(&hpack, 30);
    let mut b = ProcToggle::new("pid", view.ordering.clone());
    b.set_value(true);
    b.handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortOrder::Pid {
                    *ord = SortOrder::RevPid;
                } else {
                    *ord = SortOrder::Pid;
                }
                true
            } else {
                false
            }
        }
    });
    ProcToggle::new("mem%", view.ordering.clone()).handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortOrder::Mem {
                    *ord = SortOrder::RevMem;
                } else {
                    *ord = SortOrder::Mem;
                }
                true
            } else {
                false
            }
        }
    });
    let mut b = ProcToggle::new("virt", view.ordering.clone());
    b.handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortOrder::Virt {
                    *ord = SortOrder::RevVirt;
                } else {
                    *ord = SortOrder::Virt;
                }
                true
            } else {
                false
            }
        }
    });
    b.set_tooltip("Virtual memory in Kb");
    ProcToggle::new("cpu%", view.ordering.clone()).handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortOrder::Cpu {
                    *ord = SortOrder::RevCpu;
                } else {
                    *ord = SortOrder::Cpu;
                }
                true
            } else {
                false
            }
        }
    });
    ProcToggle::new("exe", view.ordering.clone()).handle({
        let ord = view.ordering.clone();
        move |_, e| {
            if e == Event::Push {
                let mut ord = ord.lock();
                if *ord == SortOrder::Exe {
                    *ord = SortOrder::RevExe;
                } else {
                    *ord = SortOrder::Exe;
                }
                true
            } else {
                false
            }
        }
    });
    hpack.end();
    let mut grp = group::Group::default();
    let mut b = browser::HoldBrowser::default();
    b.clear_visible_focus();
    b.set_text_size(app::font_size() - 2);
    b.set_color(Color::color_average(b.color(), Color::Background, 0.1));
    b.set_selection_color(SEL_BLUE);
    b.set_scrollbar_size(5);
    b.scrollbar()
        .set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
    b.hscrollbar()
        .set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
    b.set_frame(FrameType::GtkDownBox);
    let widths = &[70, 70, 70, 70, 70];
    b.set_column_widths(widths);
    b.set_column_char('\t');
    let mut ps = vec![];
    for (pid, process) in sys.processes() {
        ps.push(Proc::new(pid, process));
    }
    ps.sort_by(|p1, p2| p1.pid.cmp(&p2.pid));
    let light_mode = view.light_mode.load(Ordering::Relaxed);
    for p in ps {
        b.add(&p.fmt(light_mode));
    }
    let mut menu = menu::MenuButton::default().with_type(menu::MenuButtonType::Popup3);
    menu.set_frame(FrameType::FlatBox);
    menu.set_text_size(app::font_size() - 2);
    menu.set_color(Color::color_average(menu.color(), Color::Background, 0.9));
    drop(sys);
    menu.add_choice("End Task\t\t");
    b.set_callback({
        let menu = menu.clone();
        move |_| {
            if app::event_mouse_button() == MouseButton::Right {
                menu.popup();
            }
        }
    });
    grp.end();
    let mut row = group::Flex::default().row();
    parent.set_size(&row, 30);
    frame::Frame::default();
    let mut btn = button::Button::default().with_label("End task");
    btn.set_frame(FrameType::RFlatBox);
    btn.set_color(BLUE);
    btn.set_selection_color(SEL_BLUE);
    btn.clear_visible_focus();
    btn.set_callback({
        let sys = view.system.clone();
        let b = b.clone();
        move |_| {
            let val = b.value();
            if val != 0 {
                if let Some(text) = b.text(val) {
                    let sys = sys.lock();
                    let v: Vec<&str> = text.split_ascii_whitespace().collect();
                    let pid = if light_mode { v[0] } else { v[1] };
                    let pid = sysinfo::Pid::from_str(pid).unwrap();
                    if let Some(p) = sys.process(pid) {
                        p.kill_with(sysinfo::Signal::Kill).unwrap();
                    }
                    drop(sys);
                }
            }
        }
    });
    frame::Frame::default();
    row.set_size(&btn, 80);
    row.end();
    menu.set_callback(move |m| {
        if let Some(v) = m.choice() {
            if v == "End Task" {
                btn.do_callback();
            }
        }
    });
    grp.resize_callback({
        let mut b = b.clone();
        move |_, x, y, w, h| {
            b.resize(x, y, w, h);
            menu.resize(x, y, w, h);
        }
    });
    let sys = Arc::new(Mutex::new(System::new_all()));
    let light_mode = view.light_mode.clone();
    let ord = view.ordering.clone();
    let cb = move || {
        if let Some(mut sys) = sys.try_lock() {
            sys.refresh_processes();
            let mut ps = vec![];
            for (pid, process) in sys.processes() {
                ps.push(Proc::new(pid, process));
            }
            ps.sort_by(|p1, p2| match *ord.lock() {
                SortOrder::Pid => p1.pid.cmp(&p2.pid),
                SortOrder::Mem => p1.memory.cmp(&p2.memory),
                SortOrder::Virt => p1.virt.cmp(&p2.virt),
                SortOrder::Cpu => p1.cpu.partial_cmp(&p2.cpu).unwrap(),
                SortOrder::Exe => p1.exe.cmp(&p2.exe),
                SortOrder::RevPid => p2.pid.cmp(&p1.pid),
                SortOrder::RevMem => p2.memory.cmp(&p1.memory),
                SortOrder::RevVirt => p2.virt.cmp(&p1.virt),
                SortOrder::RevCpu => p2.cpu.partial_cmp(&p1.cpu).unwrap(),
                SortOrder::RevExe => p2.exe.cmp(&p1.exe),
            });
            let light_mode = light_mode.load(Ordering::Relaxed);
            for (i, p) in ps.iter().enumerate() {
                b.set_text(i as i32 + 1, &p.fmt(light_mode));
            }
            app::awake();
        }
    };
    Some(Box::new(cb))
}
