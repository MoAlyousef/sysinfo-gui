use super::MyView;
use crate::gui::styles::colors::*;
use fltk::{enums::*, prelude::*, *};
use fltk_extras::card::Card;
use fltk_extras::dial::HalfDial;
use parking_lot::Mutex;
use std::sync::Arc;
use sysinfo::{Disks, Networks, System};

pub fn general(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut sys = view.system.lock();
    sys.refresh_all();
    let mem = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.;
    let mut total_space = 0;
    let mut avail_space = 0;
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        total_space += disk.total_space();
        avail_space += disk.available_space();
    }
    let used_space = ((total_space - avail_space) as f64 * 100. / total_space as f64) as i32;
    let mut cpu_usage = 0.;
    for process in sys.processes().values() {
        cpu_usage += process.cpu_usage();
    }
    let mut dials = vec![];
    let row = group::Flex::default().row();
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
    row.end();
    let mut row = group::Flex::default().row();
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
            System::name().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "Kernel version: {}",
            System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned()),
        ));
    frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "OS version: {}",
            System::os_version().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "Long OS version: {}",
            System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "Host name: {}",
            System::host_name().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    t.end();
    let mut vpack = group::Flex::default().column();
    vpack.set_pad(30);
    row.fixed(&vpack, 160);
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
    row.end();
    drop(sys);
    let dials = Arc::new(Mutex::new(dials));
    let sys = Arc::new(Mutex::new(System::new_all()));
    let networks = Arc::new(Mutex::new(Networks::new_with_refreshed_list()));
    let cb = move || {
        if let Some(mut sys) = sys.try_lock() {
            sys.refresh_all();
            let mem = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.;
            let mut total_space = 0;
            let mut avail_space = 0;
            let ds = Disks::new_with_refreshed_list();
            for disk in ds.list() {
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
            if let Some(mut nets) = networks.try_lock() {
                nets.refresh(true);
                for (_name, data) in nets.iter() {
                    total_recv += data.total_received();
                    total_transm += data.total_transmitted();
                }
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
