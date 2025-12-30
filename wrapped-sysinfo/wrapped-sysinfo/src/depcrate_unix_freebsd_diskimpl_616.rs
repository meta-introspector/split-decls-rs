// Generated macro for impl_616 (impl)
macro_rules! Depcrate_unix_freebsd_diskimpl_616 {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"impl_616"}
// Dependencies: {}
impl GetValues for & mut DiskInner { fn update_old (& mut self) { self . old_read_bytes = self . read_bytes ; self . old_written_bytes = self . written_bytes ; } fn get_read (& mut self) -> & mut u64 { & mut self . read_bytes } fn get_written (& mut self) -> & mut u64 { & mut self . written_bytes } fn dev_id (& self) -> Option < & String > { self . dev_id . as_ref () } }
};
}
