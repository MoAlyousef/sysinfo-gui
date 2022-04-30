use super::SYSTEM;
use crate::{
    logic::{message::SysMsg, CHAN, SLEEP},
    widgets::{Card, Dial},
};
use fltk::{enums::*, prelude::*, *};
use std::sync::{atomic::Ordering, Arc, Mutex};
use sysinfo::ProcessorExt;
use sysinfo::SystemExt;

pub fn proc() -> group::Pack {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(60, 60, 0, 0, None);
    let mut grp = group::Pack::new(60, 60, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    let mut dials = vec![];
    for proc in sys.processors() {
        let mut hpack = group::Pack::default()
            .with_size(600, 130)
            .with_type(group::PackType::Horizontal);
        hpack.set_spacing(50);
        let t = Card::new(0, 0, 300, 130, proc.name());
        t.begin();
        let pack = group::Pack::default().with_size(300, 130).center_x(&*t);
        let mut f = frame::Frame::default()
            .with_size(0, 60)
            .with_label(&format!("Vendor ID: {}", proc.vendor_id()));
        f.set_label_color(Color::White);
        let mut f = frame::Frame::default()
            .with_size(0, 60)
            .with_label(&format!("Brand: {}", proc.brand()));
        f.set_label_color(Color::White);
        pack.end();
        t.end();
        let mut g = group::Group::default().with_size(130, 130);
        let mut dial = Dial::new(0, 0, 100, 100, "Cpu Usage %").center_of_parent();
        dial.modifiable(false);
        dial.set_value(proc.cpu_usage() as i32);
        dial.set_selection_color(Color::from_hex(0x82c74b));
        dials.push(dial);
        g.make_resizable(false);
        g.end();
        hpack.end();
    }
    grp.end();
    drop(sys);
    let dials = Arc::new(Mutex::new(dials));
    std::thread::spawn({
        let grp = grp.clone();
        move || {
            while grp.visible() {
                let r = &CHAN.1;
                if let Ok(msg) = r.try_recv() {
                    if let SysMsg::CpuUsage(num, val) = msg {
                        dials.lock().unwrap()[num as usize].set_value(val)
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
