// Generated macro for CpuRefreshKind (struct)
macro_rules! Depcrate_common_systemCpuRefreshKind {
() => {
// Module: crate::common::system
// Provides: {"CpuRefreshKind"}
// Dependencies: {}
# [doc = " Used to determine what you want to refresh specifically on the [`Cpu`] type."] # [doc = ""] # [doc = " ⚠\u{fe0f} Just like all other refresh types, ruling out a refresh doesn't assure you that"] # [doc = " the information won't be retrieved if the information is accessible without needing"] # [doc = " extra computation."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::{CpuRefreshKind, System};"] # [doc = ""] # [doc = " let mut system = System::new();"] # [doc = ""] # [doc = " // We don't want to update all the CPU information."] # [doc = " system.refresh_cpu_specifics(CpuRefreshKind::everything().without_frequency());"] # [doc = ""] # [doc = " for cpu in system.cpus() {"] # [doc = "     assert_eq!(cpu.frequency(), 0);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`Cpu`]: crate::Cpu"] # [derive (Clone , Copy , Debug , Default , PartialEq , Eq)] pub struct CpuRefreshKind { cpu_usage : bool , frequency : bool , }
};
}
