// Generated macro for impl_193 (impl)
macro_rules! Depcrate_common_systemimpl_193 {
() => {
// Module: crate::common::system
// Provides: {"impl_193"}
// Dependencies: {}
impl CpuRefreshKind { # [doc = " Creates a new `CpuRefreshKind` with every refresh set to `false`."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::CpuRefreshKind;"] # [doc = ""] # [doc = " let r = CpuRefreshKind::nothing();"] # [doc = ""] # [doc = " assert_eq!(r.frequency(), false);"] # [doc = " assert_eq!(r.cpu_usage(), false);"] # [doc = " ```"] pub fn nothing () -> Self { Self :: default () } # [doc = " Creates a new `CpuRefreshKind` with every refresh set to `true`."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::CpuRefreshKind;"] # [doc = ""] # [doc = " let r = CpuRefreshKind::everything();"] # [doc = ""] # [doc = " assert_eq!(r.frequency(), true);"] # [doc = " assert_eq!(r.cpu_usage(), true);"] # [doc = " ```"] pub fn everything () -> Self { Self { cpu_usage : true , frequency : true , } } impl_get_set ! (CpuRefreshKind , cpu_usage , with_cpu_usage , without_cpu_usage) ; impl_get_set ! (CpuRefreshKind , frequency , with_frequency , without_frequency) ; }
};
}
