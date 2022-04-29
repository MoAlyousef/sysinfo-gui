#[derive(Debug, Clone)]
pub enum SysMsg {
    CpuUsage(i32, i32),
    Mem(u64, u64),
    Swap(u64, u64),
}
