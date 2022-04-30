#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Message {
    General,
    Disks,
    Proc,
    Memory,
    Procs,
    Net,
    Settings,
}
