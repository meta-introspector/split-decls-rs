// Generated macro for impl_612 (impl)
macro_rules! Depcrate_unix_freebsd_diskimpl_612 {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"impl_612"}
// Dependencies: {}
impl DiskInner { pub (crate) fn kind (& self) -> DiskKind { DiskKind :: Unknown (- 1) } pub (crate) fn name (& self) -> & OsStr { & self . name } pub (crate) fn file_system (& self) -> & OsStr { & self . file_system } pub (crate) fn mount_point (& self) -> & Path { & self . mount_point } pub (crate) fn total_space (& self) -> u64 { self . total_space } pub (crate) fn available_space (& self) -> u64 { self . available_space } pub (crate) fn is_removable (& self) -> bool { self . is_removable } pub (crate) fn is_read_only (& self) -> bool { self . is_read_only } pub (crate) fn refresh_specifics (& mut self , refresh_kind : DiskRefreshKind) -> bool { refresh_disk (self , refresh_kind) } pub (crate) fn usage (& self) -> DiskUsage { DiskUsage { read_bytes : self . read_bytes . saturating_sub (self . old_read_bytes) , total_read_bytes : self . read_bytes , written_bytes : self . written_bytes . saturating_sub (self . old_written_bytes) , total_written_bytes : self . written_bytes , } } }
};
}
