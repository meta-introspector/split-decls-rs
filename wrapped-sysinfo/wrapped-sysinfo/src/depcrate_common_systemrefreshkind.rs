// Generated macro for RefreshKind (struct)
macro_rules! Depcrate_common_systemRefreshKind {
() => {
// Module: crate::common::system
// Provides: {"RefreshKind"}
// Dependencies: {}
# [doc = " Used to determine what you want to refresh specifically on the [`System`][crate::System] type."] # [doc = ""] # [doc = " ⚠\u{fe0f} Just like all other refresh types, ruling out a refresh doesn't assure you that"] # [doc = " the information won't be retrieved if the information is accessible without needing"] # [doc = " extra computation."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::{RefreshKind, System};"] # [doc = ""] # [doc = " // We want everything except memory."] # [doc = " let mut system = System::new_with_specifics(RefreshKind::everything().without_memory());"] # [doc = ""] # [doc = " assert_eq!(system.total_memory(), 0);"] # [doc = " # if sysinfo::IS_SUPPORTED_SYSTEM && !cfg!(feature = \"apple-sandbox\") {"] # [doc = " assert!(system.processes().len() > 0);"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Copy , Debug , Default , PartialEq , Eq)] pub struct RefreshKind { processes : Option < ProcessRefreshKind > , memory : Option < MemoryRefreshKind > , cpu : Option < CpuRefreshKind > , }
};
}
