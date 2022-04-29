#[derive(Debug, Clone)]
pub enum SysMsg {
    CpuUsage(i32, i32),
    UsedMem(i32),
    UsedSwap(i32),
}
