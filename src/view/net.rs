use super::MyView;
use crate::utils;
use fltk::{prelude::*, *};
use fltk_extras::card::Card;
use parking_lot::Mutex;
use std::sync::Arc;
use sysinfo::Networks;

pub fn network(_view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let networks = Networks::new_with_refreshed_list();
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
    for (name, data) in &networks {
        let t = Card::default().with_size(300, 130).with_label(name);
        t.begin();
        let p = group::Pack::default()
            .with_size(280, 130)
            .center_of_parent();
        let f = frame::Frame::default()
            .with_size(80, 60)
            .with_label(&format!(
                "Received: {} B - Transmitted: {} B",
                data.received(),
                data.transmitted()
            ));
        frames.push(f);
        let f = frame::Frame::default()
            .with_size(80, 60)
            .with_label(&format!(
                "Total Received: {:.02} MiB - Total Transmitted: {:.02} MiB",
                data.total_received() as f64 / 2_f64.powf(20.),
                data.total_transmitted() as f64 / 2_f64.powf(20.)
            ));
        frames.push(f);
        p.end();
        t.end();
    }
    vpack.end();
    scroll.end();
    let frames = Arc::new(Mutex::new(frames));
    let networks = Arc::new(Mutex::new(Networks::new_with_refreshed_list()));
    let cb = move || {
        if let Some(mut networks) = networks.try_lock() {
            networks.refresh(true);
            let mut i = 0;
            for (_name, data) in networks.iter() {
                frames.lock()[i].set_label(&format!(
                    "Received: {} B - Transmitted: {} B",
                    data.received(),
                    data.transmitted()
                ));
                frames.lock()[i + 1].set_label(&format!(
                    "Total Received: {:.02} MiB - Total Transmitted: {:.02} MiB",
                    data.total_received() as f64 / 2_f64.powf(20.),
                    data.total_transmitted() as f64 / 2_f64.powf(20.)
                ));
                i += 2;
            }
            app::awake();
        }
    };
    Some(Box::new(cb))
}
