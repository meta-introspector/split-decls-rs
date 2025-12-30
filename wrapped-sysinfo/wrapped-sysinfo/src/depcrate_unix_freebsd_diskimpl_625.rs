// Generated macro for impl_625 (impl)
macro_rules! Depcrate_unix_freebsd_diskimpl_625 {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"impl_625"}
// Dependencies: {}
impl GeomSnapshot { unsafe fn new () -> Option < Self > { match NonNull :: new (unsafe { geom_stats_snapshot_get () }) { Some (n) => Some (Self (n)) , None => { sysinfo_debug ! ("geom_stats_snapshot_get failed") ; None } } } fn iter (& mut self) -> GeomSnapshotIter < '_ > { GeomSnapshotIter (self) } fn reset (& mut self) { unsafe { geom_stats_snapshot_reset (self . 0 . as_mut ()) } } }
};
}
