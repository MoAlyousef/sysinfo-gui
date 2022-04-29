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
        let mut i = 0;
        for proc in sys.processors() {
            sender
                .try_send(SysMsg::CpuUsage(i, proc.cpu_usage() as i32))
                .ok();
            i += 1;
        }
        sender
            .try_send(SysMsg::UsedMem(
                (sys.used_memory() as f64 / sys.total_memory() as f64 * 100.) as i32,
            ))
            .ok();
        sender
            .try_send(SysMsg::UsedSwap(
                (sys.used_swap() as f64 / sys.total_swap() as f64 * 100.) as i32,
            ))
            .ok();
        std::thread::sleep(std::time::Duration::from_millis(
            SLEEP.load(Ordering::Relaxed),
        ));
    });
}
