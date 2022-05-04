use super::{MyView, SortOrder};
use crate::styles::colors::*;
use fltk::{app::MouseButton, enums::*, prelude::*, *};
use std::str::FromStr;
use std::sync::atomic::Ordering;
use sysinfo::ProcessExt;
use sysinfo::SystemExt;

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
            exe: format!("{}", proc.exe().display()),
            // total_written_bytes: 0,
            // written_bytes: 0,
            // total_read_bytes: 0,
            // read_bytes: 0,
        }
    }
    pub fn fmt(&self, light: bool) -> String {
        if !light {
            format!(
                "@C255 {}\t@C255 {:.02}\t@C255 {:.02}\t@C255 {:.02}\t@C255{}",
                self.pid,
                self.memory as f64 / 2_f64.powf(20.),
                self.virt as f64 / 2_f64.powf(20.),
                self.cpu,
                self.exe
            )
        } else {
            format!(
                " {}\t {:.02}\t {:.02}\t {:.02}\t{}",
                self.pid,
                self.memory as f64 / 2_f64.powf(20.),
                self.virt as f64 / 2_f64.powf(20.),
                self.cpu,
                self.exe
            )
        }
    }
}

pub fn procs(view: &MyView) -> group::Pack {
    let mut ord = view.ordering.lock();
    *ord = SortOrder::Pid;
    drop(ord);
    let mut sys = view.system.lock();
    sys.refresh_processes();
    let grp = group::Pack::default()
        .with_size(700, 500)
        .center_of_parent();
    let hpack = group::Pack::default()
        .with_size(0, 30)
        .with_type(group::PackType::Horizontal);
    let mut b = button::RadioButton::default()
        .with_size(70, 0)
        .with_label("pid")
        .with_align(Align::Left | Align::Inside);
    b.set_down_frame(FrameType::FlatBox);
    b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
    b.clear_visible_focus();
    b.set_label_size(app::font_size() + 1);
    b.set_value(true);
    let ord = view.ordering.clone();
    b.handle(move |_, e| {
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
    });
    b.set_frame(FrameType::FlatBox);
    let mut b = button::RadioButton::default()
        .with_size(70, 0)
        .with_label("mem")
        .with_align(Align::Left | Align::Inside);
    b.set_down_frame(FrameType::FlatBox);
    b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
    b.clear_visible_focus();
    b.set_label_size(app::font_size() + 1);
    let ord = view.ordering.clone();
    b.handle(move |_, e| {
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
    });
    b.set_frame(FrameType::FlatBox);
    let mut b = button::RadioButton::default()
        .with_size(70, 0)
        .with_label("virt")
        .with_align(Align::Left | Align::Inside);
    b.set_down_frame(FrameType::FlatBox);
    b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
    b.clear_visible_focus();
    b.set_label_size(app::font_size() + 1);
    let ord = view.ordering.clone();
    b.handle(move |_, e| {
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
    });
    b.set_frame(FrameType::FlatBox);
    let mut b = button::RadioButton::default()
        .with_size(70, 0)
        .with_label("cpu")
        .with_align(Align::Left | Align::Inside);
    b.set_down_frame(FrameType::FlatBox);
    b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
    b.clear_visible_focus();
    b.set_label_size(app::font_size() + 1);
    b.set_frame(FrameType::FlatBox);
    let ord = view.ordering.clone();
    b.handle(move |_, e| {
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
    });
    let mut b = button::RadioButton::default()
        .with_size(700 - 280, 0)
        .with_label("exe")
        .with_align(Align::Left | Align::Inside);
    b.set_down_frame(FrameType::FlatBox);
    b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
    b.clear_visible_focus();
    b.set_label_size(app::font_size() + 1);
    let ord = view.ordering.clone();
    b.handle(move |_, e| {
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
    });
    b.set_frame(FrameType::FlatBox);
    hpack.end();
    let mut b = browser::HoldBrowser::default().with_size(0, 500 - 30);
    b.clear_visible_focus();
    b.set_text_size(14);
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
    menu.set_color(Color::color_average(menu.color(), Color::Background, 0.9));
    drop(sys);
    let sys = view.system.clone();
    menu.add("End Task\t\t", Shortcut::None, menu::MenuFlag::Normal, {
        let b = b.clone();
        move |_| {
            let val = b.value();
            if val != 0 {
                if let Some(text) = b.text(val) {
                    let sys = sys.lock();
                    let v: Vec<&str> = text.split_ascii_whitespace().collect();
                    let pid = sysinfo::Pid::from_str(v[1]).unwrap();
                    if let Some(p) = sys.process(pid) {
                        p.kill_with(sysinfo::Signal::Kill).unwrap();
                    }
                    drop(sys);
                }
            }
        }
    });
    b.set_callback(move |_| {
        if app::event_mouse_button() == MouseButton::Right {
            menu.popup();
        }
    });
    let sys = view.system.clone();
    let sleep = view.sleep.clone();
    let light_mode = view.light_mode.clone();
    let ord = view.ordering.clone();
    std::thread::spawn({
        let grp = grp.clone();
        move || {
            while grp.visible() {
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
                    std::thread::sleep(std::time::Duration::from_millis(
                        sleep.load(Ordering::Relaxed),
                    ));
                    drop(sys);
                }
            }
        }
    });
    grp
}
