pub mod cpu;
pub mod disk;
pub mod general;
pub mod mem;
pub mod net;
pub mod procs;
pub mod settings;

use crate::gui::{message::Message, View};
use fltk::group::Pack;
use parking_lot::Mutex;
use std::sync::atomic::AtomicU64;
use sysinfo::{System, SystemExt};

lazy_static::lazy_static! {
    pub static ref SYSTEM: Mutex<System> = {
        let mut sys = System::new_all();
        sys.refresh_all();
        Mutex::new(sys)
    };
    pub static ref SYSTEM_LOOP: Mutex<System> = {
        let mut sys = System::new_all();
        sys.refresh_all();
        Mutex::new(sys)
    };
    pub static ref SLEEP: AtomicU64= AtomicU64::new(100);
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
            Message::Procs => procs::procs(),
            Message::Net => net::network(),
            Message::Settings => settings::settings(),
        }
    }
}
