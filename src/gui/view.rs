use fltk::group::Pack;

pub trait View {
    fn view(&self, msg: super::message::Message) -> Pack;
}
