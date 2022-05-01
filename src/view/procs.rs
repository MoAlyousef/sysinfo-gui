use super::{SLEEP, SYSTEM, SYSTEM_LOOP};
use crate::{
    gui::{message::Message, View},
    widgets::{Card, HollowRoundToggle, RoundToggle, Toggle},
};
use fltk::{enums::*, prelude::*, *};
use std::sync::{atomic::Ordering};
use sysinfo::ProcessExt;
use sysinfo::SystemExt;

pub fn procs() -> group::Pack {
    let mut sys = SYSTEM.lock();
    sys.refresh_all();
    let mut grp = group::Pack::default()
        .with_size(700, 500)
        .center_of_parent();
    grp.set_spacing(40);
    let mut b = browser::HoldBrowser::default_fill();
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
            "{}\t{:.02}\t{:.02}\t{:.02}\t{:?}",
            pid,
            process.memory() as f64/ 2_f64.powf(20.),
            process.virtual_memory() as f64/ 2_f64.powf(20.),
            process.cpu_usage(),
            process.exe()
        ));
    }
    grp.end();
    drop(sys);

    std::thread::spawn({
        let grp = grp.clone();
        move || {
            while grp.visible() {
                if let Some(mut sys) = SYSTEM_LOOP.try_lock() {
                    sys.refresh_all();
                    b.clear();
                    for (pid, process) in sys.processes() {
                        // let sysinfo::DiskUsage {
                        //     total_written_bytes,
                        //     written_bytes,
                        //     total_read_bytes,
                        //     read_bytes,
                        // } = process.disk_usage();
                        b.add(&format!(
                            "{}\t{:.02}\t{:.02}\t{:.02}\t{:?}",
                            pid,
                            process.memory() as f64/ 2_f64.powf(20.),
                            process.virtual_memory() as f64/ 2_f64.powf(20.),
                            process.cpu_usage(),
                            process.exe()
                        ));
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
