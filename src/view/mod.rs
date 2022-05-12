pub mod cpu;
pub mod disk;
pub mod general;
pub mod mem;
pub mod net;
pub mod procs;
pub mod settings;

use crate::gui::{message::Message, View};
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
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
    sleep: Arc<AtomicU64>,
    light_mode: Arc<AtomicBool>,
    ordering: Arc<Mutex<SortOrder>>,
}

impl Default for MyView {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let system = Arc::new(Mutex::new(sys));
        Self {
            system,
            sleep: Arc::new(AtomicU64::from(200)),
            light_mode: Arc::new(AtomicBool::from(false)),
            ordering: Arc::new(Mutex::new(SortOrder::Pid)),
        }
    }
}

impl View for MyView {
    fn view(&self, msg: Message) -> Option<Box<dyn FnMut() + Send>> {
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
    fn sleep_duration(&self) -> u64 {
        self.sleep.load(Ordering::Relaxed)
    }
}

impl MyView {
    pub fn general(&self) -> Option<Box<dyn FnMut() + Send>> {
        general::general(self)
    }
    pub fn memory(&self) -> Option<Box<dyn FnMut() + Send>> {
        mem::memory(self)
    }
    pub fn settings(&self) -> Option<Box<dyn FnMut() + Send>> {
        settings::settings(self)
    }
    pub fn network(&self) -> Option<Box<dyn FnMut() + Send>> {
        net::network(self)
    }
    pub fn cpu(&self) -> Option<Box<dyn FnMut() + Send>> {
        cpu::proc(self)
    }
    pub fn disks(&self) -> Option<Box<dyn FnMut() + Send>> {
        disk::disks(self)
    }
    pub fn procs(&self) -> Option<Box<dyn FnMut() + Send>> {
        procs::procs(self)
    }
}
