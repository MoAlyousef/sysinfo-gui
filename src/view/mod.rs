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
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;
use sysinfo::{System, SystemExt};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum SortOrder {
    Pid,
    Mem,
    Virt,
    Cpu,
    Exe,
    RevPid,
    RevMem,
    RevVirt,
    RevCpu,
    RevExe,
}

pub struct MyView {
    system: Arc<Mutex<System>>,
    system2: Arc<Mutex<System>>,
    sleep: Arc<AtomicU64>,
    light_mode: Arc<AtomicBool>,
    ordering: Arc<Mutex<SortOrder>>,
}

impl Default for MyView {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let system = Arc::new(Mutex::new(sys));
        let mut sys2 = System::new_all();
        sys2.refresh_all();
        let system2 = Arc::new(Mutex::new(sys2));
        Self {
            system,
            system2,
            sleep: Arc::new(AtomicU64::from(100)),
            light_mode: Arc::new(AtomicBool::from(false)),
            ordering: Arc::new(Mutex::new(SortOrder::Pid)),
        }
    }
}

impl View for MyView {
    fn view(&self, msg: Message) -> Pack {
        match msg {
            Message::General => self.general(),
            Message::Disks => self.disks(),
            Message::Proc => self.cpu(),
            Message::Memory => self.memory(),
            Message::Procs => self.procs(),
            Message::Net => self.network(),
            Message::Settings => self.settings(),
        }
    }
}

impl MyView {
    pub fn general(&self) -> Pack {
        general::general(self)
    }
    pub fn memory(&self) -> Pack {
        mem::memory(self)
    }
    pub fn settings(&self) -> Pack {
        settings::settings(self)
    }
    pub fn network(&self) -> Pack {
        net::network(self)
    }
    pub fn cpu(&self) -> Pack {
        cpu::proc(self)
    }
    pub fn disks(&self) -> Pack {
        disk::disks(self)
    }
    pub fn procs(&self) -> Pack {
        procs::procs(self)
    }
}
