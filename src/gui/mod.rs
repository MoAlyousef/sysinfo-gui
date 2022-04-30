pub mod app;
pub mod message;

use fltk::group::Pack;

pub trait View {
    fn view(&self, msg: message::Message) -> Pack;
}
