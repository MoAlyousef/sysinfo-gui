use super::MyView;
use fltk::{enums::*, prelude::*, *};
use fltk_extras::card::Card;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use sysinfo::CpuExt;
use sysinfo::System;
use sysinfo::SystemExt;

mod cpu_color {
    #![allow(non_upper_case_globals)]
    use fltk::enums::Color;

    const Col0: Color = Color::from_rgb(255, 0, 0);
    const Col1: Color = Color::from_rgb(255, 127, 0);
    const Col2: Color = Color::from_rgb(255, 212, 0);
    const Col3: Color = Color::from_rgb(255, 255, 0);
    const Col4: Color = Color::from_rgb(191, 255, 0);
    const Col5: Color = Color::from_rgb(106, 255, 0);
    const Col6: Color = Color::from_rgb(0, 234, 255);
    const Col7: Color = Color::from_rgb(0, 149, 255);
    const Col8: Color = Color::from_rgb(0, 64, 255);
    const Col9: Color = Color::from_rgb(170, 0, 255);
    const Col10: Color = Color::from_rgb(255, 0, 170);
    const Col11: Color = Color::from_rgb(237, 185, 185);
    const Col12: Color = Color::from_rgb(231, 233, 185);
    const Col13: Color = Color::from_rgb(185, 237, 224);
    const Col14: Color = Color::from_rgb(185, 215, 237);
    const Col15: Color = Color::from_rgb(220, 185, 237);
    const Col16: Color = Color::from_rgb(143, 35, 35);
    const Col17: Color = Color::from_rgb(143, 106, 35);
    const Col18: Color = Color::from_rgb(79, 143, 35);
    const Col19: Color = Color::from_rgb(35, 98, 143);
    const Col20: Color = Color::from_rgb(107, 35, 143);
    const Col21: Color = Color::from_rgb(115, 115, 115);
    const Col22: Color = Color::from_rgb(204, 204, 204);

    pub fn by_index(idx: u8) -> Color {
        match idx {
            0 => Col0,
            1 => Col1,
            2 => Col2,
            3 => Col3,
            4 => Col4,
            5 => Col5,
            6 => Col6,
            7 => Col7,
            8 => Col8,
            9 => Col9,
            10 => Col10,
            11 => Col11,
            12 => Col12,
            13 => Col13,
            14 => Col14,
            15 => Col15,
            16 => Col16,
            17 => Col17,
            18 => Col18,
            19 => Col19,
            20 => Col20,
            21 => Col21,
            22 => Col22,
            _ => Color::by_index(idx),
        }
    }
}

pub fn proc(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut sys = view.system.lock();
    sys.refresh_cpu();
    let first = sys.cpus().first().unwrap();
    let vendor_id = first.vendor_id().to_string();
    let t = Card::default().with_label(first.brand());
    let mut parent = group::Flex::from_dyn_widget(&t.parent().unwrap()).unwrap();
    parent.set_size(&*t, 60);
    t.begin();
    let mut f = frame::Frame::default().with_size(80, 30).center_of_parent();
    t.end();
    let g = group::Group::default().with_size(400, 300);
    let mut num_cpus = 0;
    let mut c = misc::Chart::default_fill();
    c.set_color(Color::color_average(c.color(), Color::Foreground, 0.9));
    c.set_bounds(0., 100.);
    c.set_type(misc::ChartType::Line);
    c.add(50.0, "50%", Color::Foreground);
    for _ in 1..20 {
        c.add(50.0, "", Color::Foreground);
    }
    c.set_text_color(Color::Foreground);
    let mut charts = vec![];
    for proc in sys.cpus() {
        let mut c = misc::Chart::default_fill();
        c.set_bounds(0., 100.);
        c.set_type(misc::ChartType::Line);
        c.set_frame(FrameType::NoBox);
        let name = proc.name().to_string();
        c.draw(move |c| {
            let row = num_cpus / 8;
            let col = num_cpus % 8;
            draw::draw_rect_fill(
                (55 * col) + c.x() + 5,
                c.y() + row * 15 + 5,
                10,
                10,
                cpu_color::by_index(num_cpus as u8),
            );
            draw::set_font(Font::Helvetica, 12);
            draw::set_draw_color(Color::Foreground);
            draw::draw_text2(
                &name,
                (55 * col) + c.x() + 20,
                c.y() + row * 15 + 5,
                10,
                10,
                Align::Left | Align::Inside,
            );
        });
        charts.push(c);
        num_cpus += 1;
    }
    f.set_label(&format!("Vendor ID: {}\nCores: {}", vendor_id, num_cpus));
    for c in &mut charts {
        for _ in 0..18 {
            c.add(0., "", Color::Red);
        }
    }
    g.end();
    let charts = Arc::new(Mutex::new(charts));
    let sys = Arc::new(Mutex::new(System::new_all()));
    let mut v = vec![];
    for _ in 0..num_cpus {
        let mut d = VecDeque::new();
        for _ in 0..20 {
            d.push_back(0.);
        }
        v.push(d);
    }
    let cb = move || {
        if let Some(mut sys) = sys.try_lock() {
            sys.refresh_cpu();
            for (i, proc) in sys.cpus().iter().enumerate() {
                v[i].push_back(proc.cpu_usage() as f64);
                v[i].pop_front();
            }
            for (count, c) in (*charts.lock()).iter_mut().enumerate() {
                for i in 1..20 {
                    let last = if let Some(val) = v[count].get(i) {
                        *val
                    } else {
                        0.
                    };
                    c.replace((i - 1) as i32, last, "", cpu_color::by_index(count as u8));
                }
            }
            app::awake();
            app::redraw();
        }
    };
    Some(Box::new(cb))
}
