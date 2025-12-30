// Generated macro for impl_974 (impl)
macro_rules! Depcrate_unknown_diskimpl_974 {
() => {
// Module: crate::unknown::disk
// Provides: {"impl_974"}
// Dependencies: {}
impl DiskInner { pub (crate) fn kind (& self) -> DiskKind { DiskKind :: Unknown (- 1) } pub (crate) fn name (& self) -> & OsStr { OsStr :: new ("") } pub (crate) fn file_system (& self) -> & OsStr { Default :: default () } pub (crate) fn mount_point (& self) -> & Path { Path :: new ("") } pub (crate) fn total_space (& self) -> u64 { 0 } pub (crate) fn available_space (& self) -> u64 { 0 } pub (crate) fn is_removable (& self) -> bool { false } pub (crate) fn is_read_only (& self) -> bool { false } pub (crate) fn refresh_specifics (& mut self , _refreshes : DiskRefreshKind) -> bool { true } pub (crate) fn usage (& self) -> DiskUsage { DiskUsage :: default () } }
};
}
