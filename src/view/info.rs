use crate::view::MyView;
use fltk::{enums::*, prelude::*, *};

const INFO: &str = r#"Sysinfo-gui is a lightweight cross-platform system-monitoring 
<a href="https://github.com/fltk-rs/fltk-rs">fltk</a> gui application based on 
<a href="https://github.com/GuillaumeGomez/sysinfo">sysinfo</a>.
<br>
Sysinfo-gui is MIT licensed.
"#;

pub fn info(_view: &MyView) -> Option<Box<dyn FnMut() + Send>> {
    let mut frame = misc::HelpView::default()
        .with_size(500, 300)
        .center_of_parent();
    frame.set_frame(FrameType::FlatBox);
    frame.set_color(frame.parent().unwrap().color());
    frame.set_value(INFO);
    frame.set_text_size(16);
    frame.set_text_font(Font::Helvetica);
    None
}
