use fltk::{prelude::*, *};

pub fn scroll_resize_cb(s: &mut group::Scroll, x: i32, y: i32, w: i32, h: i32) {
    let mut c = s.child(0).unwrap();
    c.resize(x, y, w, h);
}

pub fn fix_scroll_cb(s: &mut group::Scroll) {
    let mut scrollbar = s.scrollbar();
    // To work around Card resizing on macos
    scrollbar.set_callback({
        let mut old_cb = scrollbar.callback();
        move |s| {
            if let Some(cb) = old_cb.as_mut() {
                (*cb)();
            }
            s.parent().unwrap().redraw();
        }
    });
}