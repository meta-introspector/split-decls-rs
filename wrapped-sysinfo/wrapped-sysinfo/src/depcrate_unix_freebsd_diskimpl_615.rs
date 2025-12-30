// Generated macro for impl_615 (impl)
macro_rules! Depcrate_unix_freebsd_diskimpl_615 {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"impl_615"}
// Dependencies: {}
impl GetValues for crate :: Disk { fn update_old (& mut self) { self . inner . update_old () } fn get_read (& mut self) -> & mut u64 { self . inner . get_read () } fn get_written (& mut self) -> & mut u64 { self . inner . get_written () } fn dev_id (& self) -> Option < & String > { self . inner . dev_id () } }
};
}
