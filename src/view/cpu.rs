use super::{SLEEP, SYSTEM, SYSTEM_LOOP};
use crate::{
    widgets::{Card}, styles::colors::GRAY,
};
use fltk::{enums::*, prelude::*, *, draw::draw_rect_fill};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::{atomic::Ordering, Arc};
use sysinfo::ProcessorExt;
use sysinfo::SystemExt;

pub fn proc() -> group::Pack {
    let mut sys = SYSTEM.lock();
    sys.refresh_all();
    let first = sys.processors().first().unwrap();
    let vendor_id = first.vendor_id().to_string();
    let mut grp = group::Pack::new(60, 60, 600, 400, None).center_of_parent();
    grp.set_spacing(40);
    let t = Card::new(0, 0, 300, 80, &first.brand());
    t.begin();
    let mut f = frame::Frame::default().with_size(80, 30).center_of_parent();
    f.set_label_color(Color::White);
    t.end();
    let g = group::Group::default().with_size(400, 300);
    let mut num_cpus = 0;
    let mut c = misc::Chart::default_fill();
    c.set_color(GRAY.darker());
    c.set_bounds(0., 100.);
    c.set_type(misc::ChartType::Line);
    let mut charts = vec![];
    for proc in sys.processors() {
        let mut c = misc::Chart::default_fill();
        c.set_bounds(0., 100.);
        c.set_type(misc::ChartType::Line);
        c.set_frame(FrameType::NoBox);
        let name = proc.name().to_string();
        c.draw(move |c| {
            draw_rect_fill((50 * num_cpus) + c.x() + 5, c.y() + 5, 10, 10, Color::by_index(num_cpus as u8 + 2));
            draw::set_font(Font::Helvetica, 10);
            draw::set_draw_color(Color::White);
            draw::draw_text2(&name, (50 * num_cpus) + c.x() + 15, c.y() + 5, 10, 10, Align::Left|Align::Inside);
        });
        charts.push(c);
        num_cpus += 1;
    }
    f.set_label(&format!("Vendor ID: {}\nCores: {}", vendor_id, num_cpus));
    drop(sys);
    for c in &mut charts {
        for _ in 0..18 {
            c.add(0., "", Color::Red);
        }
    }
    g.end();
    grp.end();
    let charts = Arc::new(Mutex::new(charts));
    std::thread::spawn({
        let grp = grp.clone();
        let charts = charts.clone();
        move || {
            let mut v = vec![];
            for _ in 0..num_cpus {
                let mut d = VecDeque::new();
                for _ in 0..20 {
                    d.push_back(0.);
                }
                v.push(d);
            }

            while grp.visible() {
                if let Some(mut sys) = SYSTEM_LOOP.try_lock() {
                    sys.refresh_all();
                    for (i, proc) in sys.processors().iter().enumerate() {
                        v[i].push_back(proc.cpu_usage() as f64);
                        v[i].pop_front();
                    }
                    let mut count = 0;
                    for c in &mut *charts.lock() {
                        for i in 1..20 {
                            let last = if let Some(val) = v[count].get(i) {
                                *val
                            } else {
                                0.
                            };
                            c.replace((i - 1) as i32, last, "", Color::by_index((count + 2) as _));
                        }
                        count += 1;
                    }
                    drop(sys);
                    app::awake();
                    app::redraw();
                    std::thread::sleep(std::time::Duration::from_millis(
                        SLEEP.load(Ordering::Relaxed),
                    ));
                }
            }
        }
    });
    grp
}
