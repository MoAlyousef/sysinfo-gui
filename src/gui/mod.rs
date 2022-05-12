pub mod app;
pub mod message;
pub mod styles;
pub mod widgets;

pub trait View {
    fn view(&self, msg: message::Message) -> Option<Box<dyn FnMut() + Send>>;
    fn sleep_duration(&self) -> u64;
}
