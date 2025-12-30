// Generated macro for MemoryRefreshKind (struct)
macro_rules! Depcrate_common_systemMemoryRefreshKind {
() => {
// Module: crate::common::system
// Provides: {"MemoryRefreshKind"}
// Dependencies: {}
# [doc = " Used to determine which memory you want to refresh specifically."] # [doc = ""] # [doc = " ⚠\u{fe0f} Just like all other refresh types, ruling out a refresh doesn't assure you that"] # [doc = " the information won't be retrieved if the information is accessible without needing"] # [doc = " extra computation."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::{MemoryRefreshKind, System};"] # [doc = ""] # [doc = " let mut system = System::new();"] # [doc = ""] # [doc = " // We don't want to update all memories information."] # [doc = " system.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());"] # [doc = ""] # [doc = " println!(\"total RAM: {}\", system.total_memory());"] # [doc = " println!(\"free RAM:  {}\", system.free_memory());"] # [doc = " ```"] # [derive (Clone , Copy , Debug , Default , PartialEq , Eq)] pub struct MemoryRefreshKind { ram : bool , swap : bool , }
};
}
