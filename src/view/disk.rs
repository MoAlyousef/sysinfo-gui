use super::MyView;
use crate::gui::styles::colors::*;
use crate::utils;
use fltk::{prelude::*, *};
use fltk_extras::card::Card;
use fltk_extras::dial::Dial;
use sysinfo::DiskExt;
use sysinfo::SystemExt;

pub fn disks(view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut sys = view.system.lock();
    sys.refresh_disks();
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
    for disk in sys.disks() {
        let mut row = group::Flex::default().row().with_size(0, 130);
        row.set_margin(10);
        let t = Card::default()
            .with_size(300, 130)
            .with_label(disk.name().to_str().unwrap());
        t.begin();
        let vpack = group::Pack::default()
            .with_size(130, 130)
            .center_of_parent();
        let mut f = frame::Frame::default()
            .with_size(80, 35)
            .with_label(disk.mount_point().to_str().unwrap());
        f.set_label_size(14);
        frame::Frame::default()
            .with_size(80, 35)
            .with_label(&format!(
                "{:?}: {} - Space: {:.02} GiB",
                disk.type_(),
                String::from_utf8(disk.file_system().to_vec()).unwrap(),
                disk.total_space() as f64 / 2_f64.powf(30.)
            ));
        frame::Frame::default()
            .with_size(80, 35)
            .with_label(&format!(
                "Removable: {}",
                if disk.is_removable() { "Yes" } else { "No" }
            ));
        vpack.end();
        t.end();
        let mut dial = Dial::default().with_label("Used space %");
        row.set_size(&*dial, 120);
        dial.modifiable(false);
        dial.set_value(
            ((disk.total_space() - disk.available_space()) as f64 * 100.
                / disk.total_space() as f64) as i32,
        );
        dial.set_selection_color(DISK_PURPLE);
        row.end();
    }
    vpack.end();
    scroll.end();
    None
}
