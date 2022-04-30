pub mod cpu;
pub mod disk;
pub mod general;
pub mod mem;
pub mod net;
pub mod settings;

use crate::gui::{message::Message, View};
use fltk::group::Pack;
use std::sync::Mutex;
use sysinfo::{System, SystemExt};

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
    fn view(&self, msg: Message) -> Pack {
        match msg {
            Message::General => general::general(),
            Message::Disks => disk::disks(),
            Message::Proc => cpu::proc(),
            Message::Memory => mem::memory(),
            Message::Net => net::network(),
            Message::Settings => settings::settings(),
        }
    }
}
