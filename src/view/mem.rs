use super::SYSTEM;
use crate::{
    logic::{message::SysMsg, CHAN, SLEEP},
    widgets::{Card, Dial},
};
use fltk::{enums::*, prelude::*, *};
use std::sync::{atomic::Ordering, Arc, Mutex};
use sysinfo::SystemExt;

pub fn memory() -> group::Pack {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(60, 60, 0, 0, None);
    let mut dials = vec![];
    let mut grp = group::Pack::new(60, 60, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    let mut hpack = group::Pack::default()
        .with_size(600, 130)
        .with_type(group::PackType::Horizontal);
    hpack.set_spacing(50);
    let t = Card::new(0, 0, 300, 60, "Memory");
    t.begin();
    let pack = group::Pack::default().with_size(300, 130).center_x(&*t);
    let mut f = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!("Total: {} GiB", sys.total_memory() / 1000000));
    f.set_label_color(Color::White);
    let mut used_mem = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!("Used: {} GiB", sys.used_memory() / 1000000));
    used_mem.set_label_color(Color::White);
    pack.end();
    t.end();
    let mut g = group::Group::default().with_size(130, 130);
    let mut dial = Dial::new(0, 0, 100, 100, "Memory Usage %").center_of_parent();
    dial.modifiable(false);
    dial.set_selection_color(Color::from_hex(0xf6a22f));
    dial.set_value((sys.used_memory() as f64 / sys.total_memory() as f64 * 100.) as i32);
    dials.push(dial);
    g.make_resizable(false);
    g.end();
    hpack.end();
    let mut hpack = group::Pack::default()
        .with_size(600, 130)
        .with_type(group::PackType::Horizontal);
    hpack.set_spacing(50);
    let t = Card::new(0, 0, 300, 60, "Swap");
    t.begin();
    let pack = group::Pack::default().with_size(300, 130).center_x(&*t);
    let mut f = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!("Total: {} GiB", sys.total_swap() / 1000000));
    f.set_label_color(Color::White);
    let mut used_swap = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!("Used: {} GiB", sys.used_swap() / 1000000));
    used_swap.set_label_color(Color::White);
    pack.end();
    t.end();
    let mut g = group::Group::default().with_size(130, 130);
    let mut dial = Dial::new(0, 0, 100, 100, "Swap Usage %").center_of_parent();
    dial.modifiable(false);
    dial.set_selection_color(Color::from_hex(0xf6a22f));
    dial.set_value((sys.used_swap() as f64 / sys.total_swap() as f64 * 100.) as i32);
    dials.push(dial);
    g.make_resizable(false);
    g.end();
    hpack.end();
    grp.end();
    let dials = Arc::new(Mutex::new(dials));
    std::thread::spawn({
        let grp = grp.clone();
        move || {
            while grp.visible() {
                let r = &CHAN.1;
                if let Ok(msg) = r.try_recv() {
                    match msg {
                        SysMsg::Mem(v, t) => {
                            dials.lock().unwrap()[0].set_value((v as f64 / t as f64 * 100.) as i32);
                            used_mem.set_label(&format!("Used: {} GiB", v / 1000000));
                        }
                        SysMsg::Swap(v, t) => {
                            dials.lock().unwrap()[1].set_value((v as f64 / t as f64 * 100.) as i32);
                            used_swap.set_label(&format!("Used: {} GiB", v / 1000000));
                        }
                        _ => (),
                    }
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
