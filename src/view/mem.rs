use super::MyView;
use crate::gui::styles::colors::MEM_YELLOW;
use fltk::{prelude::*, *};
use fltk_extras::card::Card;
use fltk_extras::dial::Dial;
use parking_lot::Mutex;
use std::sync::Arc;
use sysinfo::System;
// sysinfo 0.37 exposes methods without extension traits

pub fn memory(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut sys = view.system.lock();
    sys.refresh_memory();
    let mut dials = vec![];
    let mut scroll = group::Scroll::default_fill().with_type(group::ScrollType::Vertical);
    scroll.resize_callback(crate::utils::scroll_resize_cb);
    scroll.set_scrollbar_size(-1);
    crate::utils::fix_scroll_cb(&mut scroll);
    let mut vpack = group::Pack::default()
        .with_size(300, 300)
        .with_type(group::PackType::Vertical)
        .center_of_parent();
    vpack.set_spacing(50);
    frame::Frame::default().with_size(0, 30);
    let mut row = group::Flex::default().with_size(0, 150).row();
    let t = Card::default().with_label("Memory").with_size(300, 130);
    t.begin();
    let pack = group::Pack::default()
        .with_size(300, 130)
        .center_of_parent();
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
    let mut dial = Dial::default().with_label("Memory Usage %");
    row.fixed(&*dial, 150);
    dial.modifiable(false);
    dial.set_selection_color(MEM_YELLOW);
    dial.set_value((sys.used_memory() as f64 / sys.total_memory() as f64 * 100.) as i32);
    dials.push(dial);
    row.end();
    let mut row = group::Flex::default().with_size(0, 150).row();
    let t = Card::default().with_label("Swap").with_size(300, 130);
    t.begin();
    let pack = group::Pack::default().with_size(300, 130);
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
    let mut dial = Dial::default().with_label("Swap Usage %");
    row.fixed(&*dial, 150);
    dial.modifiable(false);
    dial.set_selection_color(MEM_YELLOW);
    dial.set_value((sys.used_swap() as f64 / sys.total_swap() as f64 * 100.) as i32);
    dials.push(dial);
    row.end();
    vpack.end();
    scroll.end();
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
