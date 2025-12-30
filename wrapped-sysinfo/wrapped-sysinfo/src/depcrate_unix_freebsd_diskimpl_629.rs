// Generated macro for impl_629 (impl)
macro_rules! Depcrate_unix_freebsd_diskimpl_629 {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"impl_629"}
// Dependencies: {}
impl < 'a > Iterator for GeomSnapshotIter < 'a > { type Item = Devstat < 'a > ; fn next (& mut self) -> Option < Self :: Item > { let raw = unsafe { geom_stats_snapshot_next (self . 0 . 0 . as_mut ()) } ; NonNull :: new (raw) . map (| devstat | Devstat { devstat , phantom : PhantomData , }) } }
};
}
