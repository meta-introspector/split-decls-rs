// Generated macro for impl_626 (impl)
macro_rules! Depcrate_unix_freebsd_diskimpl_626 {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"impl_626"}
// Dependencies: {}
impl Drop for GeomSnapshot { fn drop (& mut self) { unsafe { geom_stats_snapshot_free (self . 0 . as_mut ()) } ; } }
};
}
