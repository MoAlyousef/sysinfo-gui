use super::{SLEEP, SYSTEM, SYSTEM_LOOP};
use crate::widgets::Card;
use fltk::{enums::*, prelude::*, *};
use parking_lot::Mutex;
use std::sync::{atomic::Ordering, Arc};
use sysinfo::NetworkExt;
use sysinfo::NetworksExt;
use sysinfo::SystemExt;

pub fn network() -> group::Pack {
    let mut sys = SYSTEM.lock();
    sys.refresh_all();
    frame::Frame::new(60, 60, 0, 0, None);
    let mut grp = group::Pack::new(60, 60, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    let mut frames = vec![];
    for comp in sys.networks().iter() {
        let t = Card::new(0, 0, 300, 130, comp.0);
        t.begin();
        let p = group::Pack::default()
            .with_size(300, 130)
            .center_of_parent();
        let mut f = frame::Frame::default()
            .with_size(80, 60)
            .with_label(&format!(
                "Received: {} B - Transmitted: {} B",
                comp.1.received(),
                comp.1.transmitted()
            ));
        f.set_label_color(Color::White);
        frames.push(f);
        let mut f = frame::Frame::default()
            .with_size(80, 60)
            .with_label(&format!(
                "Total Received: {:.02} MiB - Total Transmitted: {:.02} MiB",
                comp.1.total_received() as f64 / 2_f64.powf(20.),
                comp.1.total_transmitted() as f64 / 2_f64.powf(20.)
            ));
        f.set_label_color(Color::White);
        frames.push(f);
        p.end();
        t.end();
    }
    drop(sys);
    grp.end();
    let frames = Arc::new(Mutex::new(frames));

    std::thread::spawn({
        let grp = grp.clone();
        move || {
            while grp.visible() {
                if let Some(mut sys) = SYSTEM_LOOP.try_lock() {
                    sys.refresh_all();
                    let mut i = 0;
                    for comp in sys.networks() {
                        frames.lock()[i].set_label(&format!(
                            "Received: {} B - Transmitted: {} B",
                            comp.1.received(),
                            comp.1.transmitted()
                        ));
                        frames.lock()[i + 1].set_label(&format!(
                            "Total Received: {:.02} MiB - Total Transmitted: {:.02} MiB",
                            comp.1.total_received() as f64 / 2_f64.powf(20.),
                            comp.1.total_transmitted() as f64 / 2_f64.powf(20.)
                        ));
                        i += 2;
                    }
                    drop(sys);
                    app::awake();
                    std::thread::sleep(std::time::Duration::from_millis(
                        SLEEP.load(Ordering::Relaxed),
                    ));
                }
            }
        }
    });
    grp
}
