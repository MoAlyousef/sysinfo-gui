use super::{SLEEP, SYSTEM, SYSTEM_LOOP};
use crate::{
    gui::{message::Message, View},
    styles::colors::*,
    widgets::{Card, HollowRoundToggle, RoundToggle, Toggle},
};
use fltk::{app::MouseButton, enums::*, prelude::*, *};
use std::str::FromStr;
use std::sync::atomic::Ordering;
use sysinfo::ProcessExt;
use sysinfo::SystemExt;

pub fn procs() -> group::Pack {
    let mut sys = SYSTEM.lock();
    sys.refresh_all();
    let mut grp = group::Pack::default()
        .with_size(700, 500)
        .center_of_parent();
    let hpack = group::Pack::default()
        .with_size(0, 30)
        .with_type(group::PackType::Horizontal);
    let mut b = button::Button::default()
        .with_size(70, 0)
        .with_label("pid")
        .with_align(Align::Left | Align::Inside);
    b.set_label_color(Color::White);
    b.set_down_frame(FrameType::FlatBox);
    b.set_frame(FrameType::FlatBox);
    let mut b = button::Button::default()
        .with_size(70, 0)
        .with_label("mem")
        .with_align(Align::Left | Align::Inside);
    b.set_label_color(Color::White);
    b.set_down_frame(FrameType::FlatBox);
    b.set_frame(FrameType::FlatBox);
    let mut b = button::Button::default()
        .with_size(70, 0)
        .with_label("virt")
        .with_align(Align::Left | Align::Inside);
    b.set_label_color(Color::White);
    b.set_down_frame(FrameType::FlatBox);
    b.set_frame(FrameType::FlatBox);
    let mut b = button::Button::default()
        .with_size(70, 0)
        .with_label("cpu")
        .with_align(Align::Left | Align::Inside);
    b.set_label_color(Color::White);
    b.set_down_frame(FrameType::FlatBox);
    b.set_frame(FrameType::FlatBox);
    let mut b = button::Button::default()
        .with_size(700 - 280, 0)
        .with_label("exe")
        .with_align(Align::Left | Align::Inside);
    b.set_label_color(Color::White);
    b.set_down_frame(FrameType::FlatBox);
    b.set_frame(FrameType::FlatBox);
    hpack.end();
    let mut b = browser::HoldBrowser::default().with_size(0, 500 - 30);
    b.set_text_size(14);
    b.set_color(GRAY.darker());
    b.set_selection_color(SEL_BLUE);
    b.set_scrollbar_size(-1);
    b.set_frame(FrameType::GtkDownBox);
    let widths = &[70, 70, 70, 70, 70];
    b.set_column_widths(widths);
    b.set_column_char('\t');
    for (pid, process) in sys.processes() {
        // let sysinfo::DiskUsage {
        //     total_written_bytes,
        //     written_bytes,
        //     total_read_bytes,
        //     read_bytes,
        // } = process.disk_usage();
        b.add(&format!(
            "@C255 {}\t@C255 {:.02}\t@C255 {:.02}\t@C255 {:.02}\t@C255{:?}",
            pid,
            process.memory() as f64 / 2_f64.powf(20.),
            process.virtual_memory() as f64 / 2_f64.powf(20.),
            process.cpu_usage(),
            process.exe()
        ));
    }
    let mut menu = menu::MenuItem::new(&["End Task\t\t"]);
    menu.set_label_color(Color::White);
    // drop(sys);
    b.set_callback(move |b| {
        if app::event_mouse_button() == MouseButton::Right {
            let coords = app::event_coords();
            if let Some(val) = menu.popup(coords.0, coords.1) {
                if let Some(val) = val.label() {
                    if val == "End Task\t\t" {
                        let val = b.value();
                        if val != 0 {
                            if let Some(text) = b.text(val) {
                                let v: Vec<&str> = text.split_ascii_whitespace().collect();
                                let pid = sysinfo::Pid::from_str(v[1]).unwrap();
                                if let Some(p) = sys.process(pid) {
                                    p.kill_with(sysinfo::Signal::Kill).unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    std::thread::spawn({
        let grp = grp.clone();
        move || {
            while grp.visible() {
                if let Some(mut sys) = SYSTEM_LOOP.try_lock() {
                    sys.refresh_all();
                    let mut i = 0;
                    for (pid, process) in sys.processes() {
                        // let sysinfo::DiskUsage {
                        //     total_written_bytes,
                        //     written_bytes,
                        //     total_read_bytes,
                        //     read_bytes,
                        // } = process.disk_usage();
                        b.set_text(
                            i + 1,
                            &format!(
                                "@C255 {}\t@C255 {:.02}\t@C255 {:.02}\t@C255 {:.02}\t@C255{:?}",
                                pid,
                                process.memory() as f64 / 2_f64.powf(20.),
                                process.virtual_memory() as f64 / 2_f64.powf(20.),
                                process.cpu_usage(),
                                process.exe()
                            ),
                        );
                        i += 1;
                    }
                    app::awake();
                    std::thread::sleep(std::time::Duration::from_millis(
                        SLEEP.load(Ordering::Relaxed),
                    ));
                    drop(sys);
                }
            }
        }
    });
    grp
}
