use crate::{
    gui::{message::Message, View},
    logic::{message::SysMsg, CHAN, SLEEP},
    widgets::{Card, Dial, Toggle},
};
use fltk::{enums::*, prelude::*, *};
use std::sync::{atomic::Ordering, Arc, Mutex};
use sysinfo::{DiskExt, NetworkExt, NetworksExt, ProcessorExt, System, SystemExt};

lazy_static::lazy_static! {
    pub static ref SYSTEM: Mutex<System> = {
        let mut sys = System::new_all();
        sys.refresh_all();
        Mutex::new(sys)
    };
}

#[derive(Default)]
pub struct MyView;

impl View for MyView {
    fn view(&self, msg: Message) -> group::Pack {
        match msg {
            Message::General => general(),
            Message::Disks => disks(),
            Message::Proc => proc(),
            Message::Memory => memory(),
            Message::Net => network(),
            Message::Settings => settings(),
        }
    }
}

fn general() -> group::Pack {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    let mem = ((sys.used_memory() + sys.used_swap()) as f64
        / (sys.total_memory() + sys.total_swap()) as f64)
        * 100.;
    let mut total_space = 0;
    let mut avail_space = 0;
    for disk in sys.disks() {
        total_space += disk.total_space();
        avail_space += disk.available_space();
    }
    let used_space = ((total_space - avail_space) as f64 * 100. / total_space as f64) as i32;
    let mut cpu_usage = 0.;
    let mut count = 0;
    for proc in sys.processors() {
        cpu_usage += proc.cpu_usage();
        count += 1;
    }
    cpu_usage = cpu_usage / count as f32;
    let mut dials = vec![];
    frame::Frame::new(60, 60, 0, 0, None);
    let mut grp = group::Pack::new(60, 60, 700, 450, None).center_of_parent();
    grp.set_spacing(40);
    let mut pack0 = group::Pack::default()
        .with_size(450, 200)
        .with_type(group::PackType::Horizontal);
    pack0.set_spacing(40);
    let mut dial = Dial::new(0, 0, 200, 200, "CPU %");
    dial.set_value(cpu_usage as i32);
    dials.push(dial);
    let mut dial = Dial::new(0, 0, 200, 200, "Memory %");
    dial.set_value(mem as i32);
    dials.push(dial);
    let mut dial = Dial::new(0, 0, 200, 200, "Disk %");
    dial.set_value(used_space);
    dials.push(dial);
    pack0.end();
    let mut pack0 = group::Pack::default()
        .with_size(450, 250)
        .with_type(group::PackType::Horizontal);
    pack0.set_spacing(10);
    let t = Card::new(0, 0, 450, 250, "System info");
    t.begin();
    let mut pack = group::Pack::default().with_size(450, 300);
    pack.set_spacing(-15);
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "System name: {}",
            &sys.name().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    f.set_label_color(Color::White);
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "Kernel version: {}",
            &sys.kernel_version()
                .unwrap_or_else(|| "<unknown>".to_owned()),
        ));
    f.set_label_color(Color::White);
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "OS version: {}",
            &sys.os_version().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    f.set_label_color(Color::White);
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "Long OS version: {}",
            &sys.long_os_version()
                .unwrap_or_else(|| "<unknown>".to_owned())
        ));
    f.set_label_color(Color::White);
    let mut f = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label(&format!(
            "Host name: {}",
            &sys.host_name().unwrap_or_else(|| "<unknown>".to_owned())
        ));
    f.set_label_color(Color::White);
    t.end();
    let mut vpack = group::Pack::default().with_size(230, 100);
    vpack.set_spacing(45);
    let t = Card::new(0, 0, 200, 100, "Download");
    t.begin();
    let mut download = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label("0")
        .center_of_parent();
    download.set_label_color(Color::White);
    t.end();
    let t = Card::new(0, 0, 200, 100, "Upload");
    t.begin();
    let mut upload = frame::Frame::default()
        .with_align(Align::Left | Align::Inside)
        .with_size(80, 60)
        .with_label("0")
        .center_of_parent();
    upload.set_label_color(Color::White);
    t.end();
    vpack.end();
    pack.end();
    t.end();
    pack0.end();
    grp.end();
    drop(sys);
    let dials = Arc::new(Mutex::new(dials));
    std::thread::spawn({
        let grp = grp.clone();
        move || {
            while grp.visible() {
                let mut sys = SYSTEM.lock().unwrap();
                sys.refresh_all();
                let mem = ((sys.used_memory() + sys.used_swap()) as f64
                    / (sys.total_memory() + sys.total_swap()) as f64)
                    * 100.;
                let mut total_space = 0;
                let mut avail_space = 0;
                for disk in sys.disks() {
                    total_space += disk.total_space();
                    avail_space += disk.available_space();
                }
                let used_space =
                    ((total_space - avail_space) as f64 * 100. / total_space as f64) as i32;
                let mut cpu_usage = 0.;
                let mut count = 0;
                for proc in sys.processors() {
                    cpu_usage += proc.cpu_usage();
                    count += 1;
                }
                cpu_usage = cpu_usage / count as f32;
                let mut total_recv = 0;
                let mut total_transm = 0;
                for comp in sys.networks().iter() {
                    total_recv += comp.1.total_received();
                    total_transm += comp.1.total_transmitted();
                }
                drop(sys);
                dials.lock().unwrap()[0].set_value(cpu_usage as i32);
                dials.lock().unwrap()[1].set_value(mem as i32);
                dials.lock().unwrap()[2].set_value(used_space);
                download.set_label(&total_recv.to_string());
                upload.set_label(&total_transm.to_string());
                app::awake();
                std::thread::sleep(std::time::Duration::from_millis(
                    SLEEP.load(Ordering::Relaxed),
                ));
            }
        }
    });
    grp
}

fn disks() -> group::Pack {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(60, 60, 0, 0, None);
    let mut grp = group::Pack::new(60, 60, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    for disk in sys.disks() {
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
            ((disk.total_space() - disk.available_space()) as f64 * 100.
                / disk.total_space() as f64) as i32,
        );
        grp.end();
        hpack.end();
    }
    grp.end();
    grp
}

fn proc() -> group::Pack {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(60, 60, 0, 0, None);
    let mut toggle = Toggle::new(600, 50, 150, 40, "Live");
    toggle.set_label_color(Color::White);
    toggle.set_label_size(16);
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
        dial.set_label_color(Color::White);
        dials.push(dial);
        g.make_resizable(false);
        g.end();
        hpack.end();
    }
    grp.end();
    let dials = Arc::new(Mutex::new(dials));
    toggle.set_callback(move |b| {
        let dials = dials.clone();
        let b = b.clone();
        std::thread::spawn(move || {
            while b.value() {
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
        });
    });
    grp
}

fn memory() -> group::Pack {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    frame::Frame::new(60, 60, 0, 0, None);
    let mut toggle = Toggle::new(600, 50, 150, 40, "Live");
    toggle.set_label_color(Color::White);
    toggle.set_label_size(16);
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
        .with_label(&format!("Total: {} Gb", sys.total_memory() / 1000000));
    f.set_label_color(Color::White);
    let mut used_mem = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!("Used: {} Gb", sys.used_memory() / 1000000));
    used_mem.set_label_color(Color::White);
    pack.end();
    t.end();
    let mut g = group::Group::default().with_size(130, 130);
    let mut dial = Dial::new(0, 0, 100, 100, "Memory Usage %").center_of_parent();
    dial.modifiable(false);
    dial.set_value((sys.used_memory() as f64 / sys.total_memory() as f64 * 100.) as i32);
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
    let mut used_swap = frame::Frame::default()
        .with_size(0, 60)
        .with_label(&format!("Used: {} Gb", sys.used_swap() / 1000000));
    used_swap.set_label_color(Color::White);
    pack.end();
    t.end();
    let mut g = group::Group::default().with_size(130, 130);
    let mut dial = Dial::new(0, 0, 100, 100, "Swap Usage %").center_of_parent();
    dial.modifiable(false);
    dial.set_value((sys.used_swap() as f64 / sys.total_swap() as f64 * 100.) as i32);
    dial.set_label_color(Color::White);
    dials.push(dial);
    g.make_resizable(false);
    g.end();
    hpack.end();
    grp.end();
    let dials = Arc::new(Mutex::new(dials));
    toggle.set_callback(move |b| {
        let dials = dials.clone();
        let mut used_mem = used_mem.clone();
        let mut used_swap = used_swap.clone();
        let b = b.clone();
        std::thread::spawn(move || {
            while b.value() {
                let r = &CHAN.1;
                if let Ok(msg) = r.try_recv() {
                    match msg {
                        SysMsg::Mem(v, t) => {
                            dials.lock().unwrap()[0].set_value((v as f64 / t as f64 * 100.) as i32);
                            used_mem.set_label(&format!("Used: {} Gb", v / 1000000));
                        }
                        SysMsg::Swap(v, t) => {
                            dials.lock().unwrap()[1].set_value((v as f64 / t as f64 * 100.) as i32);
                            used_swap.set_label(&format!("Used: {} Gb", v / 1000000));
                        }
                        _ => (),
                    }
                    app::awake();
                    std::thread::sleep(std::time::Duration::from_millis(
                        SLEEP.load(Ordering::Relaxed),
                    ));
                }
            }
        });
    });
    grp
}

fn network() -> group::Pack {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_all();
    let mut toggle = Toggle::new(600, 50, 150, 40, "Live");
    toggle.set_label_color(Color::White);
    toggle.set_label_size(16);
    frame::Frame::new(60, 60, 0, 0, None);
    let mut grp = group::Pack::new(60, 60, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    for comp in sys.networks().iter() {
        let t = Card::new(0, 0, 300, 130, comp.0);
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
    drop(sys);
    grp.end();
    toggle.set_callback({
        let grp = grp.clone();
        move |b| {
            let b = b.clone();
            let mut grp = grp.clone();
            std::thread::spawn(move || {
                while b.value() {
                    let mut sys = SYSTEM.lock().unwrap();
                    sys.refresh_all();
                    grp.clear();
                    grp.begin();
                    for comp in sys.networks().iter() {
                        let t = Card::new(0, 0, 300, 130, comp.0);
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
                    drop(sys);
                    grp.end();
                    grp.redraw();
                    app::awake();
                    std::thread::sleep(std::time::Duration::from_millis(
                        SLEEP.load(Ordering::Relaxed),
                    ));
                }
            });
        }
    });
    grp
}

fn settings() -> group::Pack {
    let grp = group::Pack::default()
        .with_size(100, 400)
        .center_of_parent();
    grp.end();
    grp
}
