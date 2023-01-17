use super::MyView;
use crate::gui::styles::colors::MEM_YELLOW;
use fltk::{prelude::*, *};
use fltk_extras::card::Card;
use fltk_extras::dial::Dial;
use parking_lot::Mutex;
use std::sync::Arc;
use sysinfo::System;
use sysinfo::SystemExt;

pub fn memory(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut sys = view.system.lock();
    sys.refresh_memory();
    frame::Frame::new(60, 60, 0, 0, None);
    let mut dials = vec![];
    let mut grp = group::Pack::default()
        .with_size(600, 400)
        .center_of_parent();
    grp.set_spacing(40);
    let mut hpack = group::Pack::default()
        .with_size(600, 130)
        .with_type(group::PackType::Horizontal);
    hpack.set_spacing(50);
    let t = Card::default().with_size(300, 60).with_label("Memory");
    t.begin();
    let pack = group::Pack::default().with_size(300, 130).center_x(&*t);
    frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!(
            "Total: {:.02} GiB",
            sys.total_memory() as f64 / 2_f64.powf(20.)
        ));
    let mut used_mem = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!(
            "Used: {:.02} GiB",
            sys.used_memory() as f64 / 2_f64.powf(20.)
        ));
    pack.end();
    t.end();
    let mut g = group::Group::default().with_size(130, 130);
    let mut dial = Dial::default()
        .with_size(100, 100)
        .with_label("Memory Usage %")
        .center_of_parent();
    dial.modifiable(false);
    dial.set_selection_color(MEM_YELLOW);
    dial.set_value((sys.used_memory() as f64 / sys.total_memory() as f64 * 100.) as i32);
    dials.push(dial);
    g.make_resizable(false);
    g.end();
    hpack.end();
    let mut hpack = group::Pack::default()
        .with_size(600, 130)
        .with_type(group::PackType::Horizontal);
    hpack.set_spacing(50);
    let t = Card::default().with_size(300, 60).with_label("Swap");
    t.begin();
    let pack = group::Pack::default().with_size(300, 130).center_x(&*t);
    frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!(
            "Total: {:.02} GiB",
            sys.total_swap() as f64 / 2_f64.powf(20.)
        ));
    let mut used_swap = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!(
            "Used: {:.02} GiB",
            sys.used_swap() as f64 / 2_f64.powf(20.)
        ));
    pack.end();
    t.end();
    let mut g = group::Group::default().with_size(130, 130);
    let mut dial = Dial::default()
        .with_size(100, 100)
        .with_label("Swap Usage %")
        .center_of_parent();
    dial.modifiable(false);
    dial.set_selection_color(MEM_YELLOW);
    dial.set_value((sys.used_swap() as f64 / sys.total_swap() as f64 * 100.) as i32);
    dials.push(dial);
    g.make_resizable(false);
    g.end();
    hpack.end();
    grp.end();
    let dials = Arc::new(Mutex::new(dials));
    let sys = Arc::new(Mutex::new(System::new_all()));
    let cb = move || {
        if let Some(mut sys) = sys.try_lock() {
            sys.refresh_memory();
            dials.lock()[0]
                .set_value((sys.used_memory() as f64 / sys.total_memory() as f64 * 100.) as i32);
            used_mem.set_label(&format!(
                "Used: {:.02} GiB",
                sys.used_memory() as f64 / 2_f64.powf(20.)
            ));
            dials.lock()[1]
                .set_value((sys.used_swap() as f64 / sys.total_swap() as f64 * 100.) as i32);
            used_swap.set_label(&format!(
                "Used: {:.02} GiB",
                sys.used_swap() as f64 / 2_f64.powf(20.)
            ));
            app::awake();
        }
    };
    Some(Box::new(cb))
}
