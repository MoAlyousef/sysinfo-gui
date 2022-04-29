pub mod message;

use crossbeam_channel::{unbounded, Receiver, Sender};
use message::SysMsg;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};
use sysinfo::{ProcessorExt, System, SystemExt};

lazy_static::lazy_static! {
    pub static ref SYSTEM: Mutex<System> = {
        let mut sys = System::new_all();
        sys.refresh_all();
        Mutex::new(sys)
    };

    pub static ref SLEEP: AtomicU64 = AtomicU64::new(30);

    pub static ref CHAN: (Sender<SysMsg>, Receiver<SysMsg>) = unbounded();
}

pub fn background_thread_spawn() {
    let sender = &CHAN.0;
    std::thread::spawn(move || loop {
        let mut sys = SYSTEM.lock().unwrap();
        sys.refresh_all();
        for (i, proc) in sys.processors().iter().enumerate() {
            sender
                .try_send(SysMsg::CpuUsage(i as i32, proc.cpu_usage() as i32))
                .ok();
        }
        sender
            .try_send(SysMsg::Mem(
                sys.used_memory(), sys.total_memory()
            ))
            .ok();
        sender
            .try_send(SysMsg::Swap(
                sys.used_swap(), sys.total_swap()
            ))
            .ok();
        std::thread::sleep(std::time::Duration::from_millis(
            SLEEP.load(Ordering::Relaxed),
        ));
    });
}
