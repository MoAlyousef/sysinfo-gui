#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Message {
    General,
    Disks,
    Proc,
    Memory,
    Procs,
    Net,
    Settings,
    Info,
}
