use crate::widgets::{Card, Dial, Toggle};
use fltk::{enums::*, prelude::*, *};
use std::sync::Mutex;

use super::message::Message;
use sysinfo::{DiskExt, NetworkExt, NetworksExt, ProcessorExt, System, SystemExt};

lazy_static::lazy_static! {
    pub static ref SYSTEM: Mutex<System> = {
        let mut sys = System::new_all();
        sys.refresh_all();
        Mutex::new(sys)
    };
}

pub fn view(message: super::message::Message) {
    match message {
        Message::General => general(),
        Message::Disks => disks(),
        Message::Proc => proc(),
        Message::Memory => memory(),
        Message::Net => network(),
        Message::Settings => settings(),
    }
}

fn general() {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(50, 50, 0, 0, None);
    let mut grp = group::Pack::new(50, 50, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    let t = Card::new(0, 0, 300, 60, "System");
    t.begin();
    let mut f = frame::Frame::default()
        .with_size(80, 60)
        .with_label(&sys.name().unwrap_or_else(|| "<unknown>".to_owned()))
        .center_of_parent();
    f.set_label_color(Color::White);
    t.end();
    let t = Card::new(0, 0, 300, 60, "Kernel version");
    t.begin();
    let mut f = frame::Frame::default()
        .with_size(80, 60)
        .with_label(
            &sys.kernel_version()
                .unwrap_or_else(|| "<unknown>".to_owned()),
        )
        .center_of_parent();
    f.set_label_color(Color::White);
    t.end();
    let t = Card::new(0, 0, 300, 60, "OS version");
    t.begin();
    let mut f = frame::Frame::default()
        .with_size(80, 60)
        .with_label(&sys.os_version().unwrap_or_else(|| "<unknown>".to_owned()))
        .center_of_parent();
    f.set_label_color(Color::White);
    t.end();
    let t = Card::new(0, 0, 300, 60, "Long OS version");
    t.begin();
    let mut f = frame::Frame::default()
        .with_size(80, 60)
        .with_label(
            &sys.long_os_version()
                .unwrap_or_else(|| "<unknown>".to_owned()),
        )
        .center_of_parent();
    f.set_label_color(Color::White);
    t.end();
    let t = Card::new(0, 0, 300, 60, "Host name");
    t.begin();
    let mut f = frame::Frame::default()
        .with_size(80, 60)
        .with_label(&sys.host_name().unwrap_or_else(|| "<unknown>".to_owned()))
        .center_of_parent();
    f.set_label_color(Color::White);
    t.end();
    grp.end();
}

fn disks() {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(50, 50, 0, 0, None);
    let mut grp = group::Pack::new(50, 50, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    for disk in sys.disks() {
        if disk.type_() == sysinfo::DiskType::HDD || disk.type_() == sysinfo::DiskType::SSD {
            let mut hpack =
                group::Pack::new(0, 0, 600, 130, None).with_type(group::PackType::Horizontal);
            hpack.set_spacing(50);
            let t = Card::new(0, 0, 300, 130, disk.name().to_str().unwrap());
            t.begin();
            let mut f = frame::Frame::default()
                .with_size(80, 60)
                .with_label(&format!(
                    "{:?}: {} - Space: {} Gb",
                    disk.type_(),
                    String::from_utf8(disk.file_system().to_vec()).unwrap(),
                    disk.total_space() / 1000000000
                ))
                .center_of_parent();
            f.set_label_color(Color::White);
            t.end();
            let grp = group::Group::default().with_size(130, 130);
            let mut dial = Dial::new(0, 0, 100, 100, "Used space %").center_of_parent();
            dial.modifiable(false);
            dial.set_label_color(Color::White);
            dial.set_value(
                ((disk.total_space() - disk.available_space()) * 100 / disk.total_space()) as _,
            );
            grp.end();
            hpack.end();
        }
    }
    grp.end();
}

fn proc() {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(50, 50, 0, 0, None);
    let mut toggle = Toggle::new(600, 50, 150, 40, "Live");
    toggle.set_label_color(Color::White);
    toggle.set_label_size(16);
    let mut grp = group::Pack::new(50, 50, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    let mut dials = vec![];
    for proc in sys.processors() {
        let mut hpack = group::Pack::default()
            .with_size(600, 130)
            .with_type(group::PackType::Horizontal);
        hpack.set_spacing(50);
        let t = Card::new(0, 0, 300, 130, &proc.name());
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
        let mut dial = Dial::new(0, 0, 100, 100, "Cpu Usage").center_of_parent();
        dial.modifiable(false);
        dial.set_value(proc.cpu_usage() as i32);
        dial.set_label_color(Color::White);
        dials.push(dial);
        g.make_resizable(false);
        g.end();
        hpack.end();
    }
    grp.end();
    toggle.set_callback(move |b| {
        while b.value() {
            let mut i = 0;
            sys.refresh_all();
            for proc in sys.processors() {
                let dial = &mut dials[i];
                dial.set_value(proc.cpu_usage() as i32);
                i += 1;
            }
            app::redraw();
            app::sleep(0.03);
            app::wait();
        }
    });
}

fn memory() {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(50, 50, 0, 0, None);
    let mut toggle = Toggle::new(600, 50, 150, 40, "Live");
    toggle.set_label_color(Color::White);
    toggle.set_label_size(16);
    let mut dials = vec![];
    let mut grp = group::Pack::new(50, 50, 600, 400, None).center_of_parent();
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
        .with_label(&format!("Total: {} Gb", sys.total_memory() / 1000000));
    f.set_label_color(Color::White);
    let mut f = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!("Used: {} Gb", sys.used_memory() / 1000000));
    f.set_label_color(Color::White);
    pack.end();
    t.end();
    let mut g = group::Group::default().with_size(130, 130);
    let mut dial = Dial::new(0, 0, 100, 100, "Memory Usage %").center_of_parent();
    dial.modifiable(false);
    dial.set_value((sys.used_memory() as f64/ sys.total_memory() as f64 * 100.) as i32);
    dial.set_label_color(Color::White);
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
        .with_label(&format!("Total: {} Gb", sys.total_swap() / 1000000));
    f.set_label_color(Color::White);
    let mut f = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!("Used: {} Gb", sys.used_swap() / 1000000));
    f.set_label_color(Color::White);
    pack.end();
    t.end();
    let mut g = group::Group::default().with_size(130, 130);
    let mut dial = Dial::new(0, 0, 100, 100, "Swap Usage %").center_of_parent();
    dial.modifiable(false);
    dial.set_value((sys.used_swap() as f64 / sys.total_swap() as f64* 100.) as i32);
    dial.set_label_color(Color::White);
    dials.push(dial);
    g.make_resizable(false);
    g.end();
    hpack.end();
    grp.end();
    toggle.set_callback(move |b| {
        while b.value() {
            sys.refresh_all();
            dials[0].set_value((sys.used_memory() as f64/ sys.total_memory() as f64 * 100.) as i32);
            dials[1].set_value((sys.used_swap() as f64 / sys.total_swap() as f64 * 100.) as i32);
            app::redraw();
            app::sleep(0.03);
            app::wait();
        }
    });
}

fn network() {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(50, 50, 0, 0, None);
    let mut grp = group::Pack::new(50, 50, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    for comp in sys.networks().iter() {
        let t = Card::new(0, 0, 300, 130, &comp.0);
        t.begin();
        let p = group::Pack::default()
            .with_size(300, 130)
            .center_of_parent();
        let mut f = frame::Frame::default()
            .with_size(80, 60)
            .with_label(&format!(
                "Received: {} - Transmitted: {}",
                comp.1.received(),
                comp.1.transmitted()
            ));
        f.set_label_color(Color::White);
        let mut f = frame::Frame::default()
            .with_size(80, 60)
            .with_label(&format!(
                "Total Received: {} - Total Transmitted: {}",
                comp.1.total_received(),
                comp.1.total_transmitted()
            ));
        f.set_label_color(Color::White);
        p.end();
        t.end();
    }
    grp.end();
}

fn settings() {}