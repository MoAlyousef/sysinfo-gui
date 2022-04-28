#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Message {
    General,
    Disks,
    Proc,
    Memory,
    Net,
    Therm,
    Settings,
    Quit,
}
