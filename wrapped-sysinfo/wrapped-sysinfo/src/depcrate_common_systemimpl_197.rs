// Generated macro for impl_197 (impl)
macro_rules! Depcrate_common_systemimpl_197 {
() => {
// Module: crate::common::system
// Provides: {"impl_197"}
// Dependencies: {}
impl RefreshKind { # [doc = " Creates a new `RefreshKind` with every refresh set to `false`/`None`."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::RefreshKind;"] # [doc = ""] # [doc = " let r = RefreshKind::nothing();"] # [doc = ""] # [doc = " assert_eq!(r.processes().is_some(), false);"] # [doc = " assert_eq!(r.memory().is_some(), false);"] # [doc = " assert_eq!(r.cpu().is_some(), false);"] # [doc = " ```"] pub fn nothing () -> Self { Self :: default () } # [doc = " Creates a new `RefreshKind` with every refresh set to `true`/`Some(...)`."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::RefreshKind;"] # [doc = ""] # [doc = " let r = RefreshKind::everything();"] # [doc = ""] # [doc = " assert_eq!(r.processes().is_some(), true);"] # [doc = " assert_eq!(r.memory().is_some(), true);"] # [doc = " assert_eq!(r.cpu().is_some(), true);"] # [doc = " ```"] pub fn everything () -> Self { Self { processes : Some (ProcessRefreshKind :: everything ()) , memory : Some (MemoryRefreshKind :: everything ()) , cpu : Some (CpuRefreshKind :: everything ()) , } } impl_get_set ! (RefreshKind , processes , with_processes , without_processes , ProcessRefreshKind) ; impl_get_set ! (RefreshKind , memory , with_memory , without_memory , MemoryRefreshKind) ; impl_get_set ! (RefreshKind , cpu , with_cpu , without_cpu , CpuRefreshKind) ; }
};
}
