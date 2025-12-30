// Generated macro for Cpu (struct)
macro_rules! Depcrate_common_systemCpu {
() => {
// Module: crate::common::system
// Provides: {"Cpu"}
// Dependencies: {}
# [doc = " Contains all the methods of the [`Cpu`][crate::Cpu] struct."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::{System, RefreshKind, CpuRefreshKind};"] # [doc = ""] # [doc = " let mut s = System::new_with_specifics("] # [doc = "     RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),"] # [doc = " );"] # [doc = ""] # [doc = " // Wait a bit because CPU usage is based on diff."] # [doc = " std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);"] # [doc = " // Refresh CPUs again to get actual value."] # [doc = " s.refresh_cpu_all();"] # [doc = ""] # [doc = " for cpu in s.cpus() {"] # [doc = "     println!(\"{}%\", cpu.cpu_usage());"] # [doc = " }"] # [doc = " ```"] pub struct Cpu { pub (crate) inner : CpuInner , }
};
}
