use super::MyView;
use crate::{
    gui::styles::colors::*,
    gui::widgets::{Card, HalfDial},
};
use fltk::{enums::*, prelude::*, *};
use parking_lot::Mutex;
use std::sync::Arc;
use sysinfo::{DiskExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

pub fn general(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut sys = view.system.lock();
    sys.refresh_all();
    let mem = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.;
    let mut total_space = 0;
    let mut avail_space = 0;
    for disk in sys.disks() {
        total_space += disk.total_space();
        avail_space += disk.available_space();
    }
    let used_space = ((total_space - avail_space) as f64 * 100. / total_space as f64) as i32;
    let mut cpu_usage = 0.;
    for process in sys.processes().values() {
        cpu_usage += process.cpu_usage();
    }
    let mut dials = vec![];
    frame::Frame::new(60, 60, 0, 0, None);
    let mut grp = group::Pack::default()
        .with_size(700, 450)
        .center_of_parent();
    grp.set_spacing(30);
    let mut pack0 = group::Pack::default()
        .with_size(450, 200)
        .with_type(group::PackType::Horizontal);
    pack0.set_spacing(40);
    let mut dial = HalfDial::default().with_size(200, 200).with_label("CPU %");
    dial.set_value(cpu_usage as i32);
    dial.set_selection_color(CPU_GREEN);
    dials.push(dial);
    let mut dial = HalfDial::default()
        .with_size(200, 200)
        .with_label("Memory %");
    dial.set_selection_color(MEM_YELLOW);
    dial.set_value(mem as i32);
    dials.push(dial);
    let mut dial = HalfDial::default().with_size(200, 200).with_label("Disk %");
    dial.set_selection_color(DISK_PURPLE);
    dial.set_value(used_space);
    dials.push(dial);
    pack0.end();
    let mut pack0 = group::Pack::default()
        .with_size(450, 250)
        .with_type(group::PackType::Horizontal);
    pack0.set_spacing(10);
    let t = Card::default()
        .with_size(450, 250)
        .with_label("System info");
    t.begin();
    let mut pack = group::Pack::default().with_size(450, 300);
    pack.set_spacing(-15);
    frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "System name: {}",
            &sys.name().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "Kernel version: {}",
            &sys.kernel_version()
                .unwrap_or_else(|| "<unknown>".to_owned()),
        ));
    frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "OS version: {}",
            &sys.os_version().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "Long OS version: {}",
            &sys.long_os_version()
                .unwrap_or_else(|| "<unknown>".to_owned())
        ));
    frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "Host name: {}",
            &sys.host_name().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    t.end();
    let mut vpack = group::Pack::default().with_size(230, 100);
    vpack.set_spacing(45);
    let t = Card::default().with_size(200, 100).with_label("Download");
    t.begin();
    let mut download = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label("0")
        .center_of_parent();
    t.end();
    let t = Card::default().with_size(200, 100).with_label("Upload");
    t.begin();
    let mut upload = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label("0")
        .center_of_parent();
    upload.set_align(Align::Center | Align::Wrap);
    t.end();
    vpack.end();
    pack.end();
    t.end();
    pack0.end();
    grp.end();
    drop(sys);
    let dials = Arc::new(Mutex::new(dials));
    let sys = Arc::new(Mutex::new(System::new_all()));
    let cb = move || {
        if let Some(mut sys) = sys.try_lock() {
            sys.refresh_all();
            let mem = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.;
            let mut total_space = 0;
            let mut avail_space = 0;
            for disk in sys.disks() {
                total_space += disk.total_space();
                avail_space += disk.available_space();
            }
            let used_space =
                ((total_space - avail_space) as f64 * 100. / total_space as f64) as i32;
            let mut cpu_usage = 0.;
            for process in sys.processes().values() {
                cpu_usage += process.cpu_usage();
            }
            let mut total_recv = 0;
            let mut total_transm = 0;
            for comp in sys.networks().iter() {
                total_recv += comp.1.total_received();
                total_transm += comp.1.total_transmitted();
            }
            dials.lock()[0].set_value(cpu_usage as i32);
            dials.lock()[1].set_value(mem as i32);
            dials.lock()[2].set_value(used_space);
            download.set_label(&format!("{:.02} MiB", total_recv as f64 / 2_f64.powf(20.)));
            upload.set_label(&format!(
                "{:.02} Mib",
                total_transm as f64 / 2_f64.powf(20.)
            ));
            app::awake();
        }
    };
    Some(Box::new(cb))
}
