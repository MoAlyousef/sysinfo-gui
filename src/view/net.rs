use super::MyView;
use crate::gui::widgets::Card;
use fltk::{prelude::*, *};
use parking_lot::Mutex;
use std::sync::{atomic::Ordering, Arc};
use sysinfo::NetworkExt;
use sysinfo::NetworksExt;
use sysinfo::System;
use sysinfo::SystemExt;

pub fn network(view: &MyView) -> group::Pack {
    let mut sys = view.system.lock();
    sys.refresh_networks();
    frame::Frame::new(60, 60, 0, 0, None);
    let mut grp = group::Pack::default()
        .with_size(600, 400)
        .center_of_parent();
    grp.set_spacing(40);
    let mut frames = vec![];
    for comp in sys.networks().iter() {
        let t = Card::default().with_size(300, 130).with_label(comp.0);
        t.begin();
        let p = group::Pack::default()
            .with_size(300, 130)
            .center_of_parent();
        let f = frame::Frame::default()
            .with_size(80, 60)
            .with_label(&format!(
                "Received: {} B - Transmitted: {} B",
                comp.1.received(),
                comp.1.transmitted()
            ));
        frames.push(f);
        let f = frame::Frame::default()
            .with_size(80, 60)
            .with_label(&format!(
                "Total Received: {:.02} MiB - Total Transmitted: {:.02} MiB",
                comp.1.total_received() as f64 / 2_f64.powf(20.),
                comp.1.total_transmitted() as f64 / 2_f64.powf(20.)
            ));
        frames.push(f);
        p.end();
        t.end();
    }
    grp.end();
    let frames = Arc::new(Mutex::new(frames));
    let sys = Arc::new(Mutex::new(System::new_all()));
    let sleep = view.sleep.clone();
    std::thread::spawn({
        move || loop {
            if let Some(mut sys) = sys.try_lock() {
                sys.refresh_networks();
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
                app::awake();
            }
            std::thread::sleep(std::time::Duration::from_millis(
                sleep.load(Ordering::Relaxed),
            ));
        }
    });
    grp
}
