pub mod app;
pub mod message;
pub mod styles;
pub mod widgets;

use fltk::group::Pack;

pub trait View {
    fn view(&self, msg: message::Message) -> Pack;
}
