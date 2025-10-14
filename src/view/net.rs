use super::MyView;
use crate::utils;
use fltk::{prelude::*, *};
use fltk_extras::card::Card;
use parking_lot::Mutex;
use std::sync::Arc;
use sysinfo::NetworkExt;
use sysinfo::NetworksExt;
use sysinfo::System;
use sysinfo::SystemExt;

pub fn network(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut sys = view.system.lock();
    sys.refresh_networks();
    let mut frames = vec![];
    let mut scroll = group::Scroll::default_fill().with_type(group::ScrollType::Vertical);
    scroll.resize_callback(utils::scroll_resize_cb);
    scroll.set_scrollbar_size(-1);
    utils::fix_scroll_cb(&mut scroll);
    let mut vpack = group::Pack::default()
        .with_size(300, 300)
        .with_type(group::PackType::Vertical)
        .center_of_parent();
    vpack.set_spacing(50);
    frame::Frame::default().with_size(0, 30);
    for comp in sys.networks().iter() {
        let t = Card::default().with_size(300, 130).with_label(comp.0);
        t.begin();
        let p = group::Pack::default()
            .with_size(280, 130)
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
    vpack.end();
    scroll.end();
    let frames = Arc::new(Mutex::new(frames));
    let sys = Arc::new(Mutex::new(System::new_all()));
    let cb = move || {
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
    };
    Some(Box::new(cb))
}
