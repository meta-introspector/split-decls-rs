// Generated macro for impl_195 (impl)
macro_rules! Depcrate_common_systemimpl_195 {
() => {
// Module: crate::common::system
// Provides: {"impl_195"}
// Dependencies: {}
impl MemoryRefreshKind { # [doc = " Creates a new `MemoryRefreshKind` with every refresh set to `false`."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::MemoryRefreshKind;"] # [doc = ""] # [doc = " let r = MemoryRefreshKind::nothing();"] # [doc = ""] # [doc = " assert_eq!(r.ram(), false);"] # [doc = " assert_eq!(r.swap(), false);"] # [doc = " ```"] pub fn nothing () -> Self { Self :: default () } # [doc = " Creates a new `MemoryRefreshKind` with every refresh set to `true`."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::MemoryRefreshKind;"] # [doc = ""] # [doc = " let r = MemoryRefreshKind::everything();"] # [doc = ""] # [doc = " assert_eq!(r.ram(), true);"] # [doc = " assert_eq!(r.swap(), true);"] # [doc = " ```"] pub fn everything () -> Self { Self { ram : true , swap : true , } } impl_get_set ! (MemoryRefreshKind , ram , with_ram , without_ram) ; impl_get_set ! (MemoryRefreshKind , swap , with_swap , without_swap) ; }
};
}
