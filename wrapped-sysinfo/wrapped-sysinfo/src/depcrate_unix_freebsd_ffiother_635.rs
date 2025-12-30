// Generated macro for other_635 (other)
macro_rules! Depcrate_unix_freebsd_ffiother_635 {
() => {
// Module: crate::unix::freebsd::ffi
// Provides: {"other_635"}
// Dependencies: {}
# [link (name = "geom")] unsafe extern "C" { pub (crate) fn geom_stats_open () -> c_int ; pub (crate) fn geom_stats_snapshot_get () -> * mut c_void ; pub (crate) fn geom_stats_snapshot_next (arg : * mut c_void) -> * mut libc :: devstat ; pub (crate) fn geom_stats_snapshot_reset (arg : * mut c_void) ; pub (crate) fn geom_stats_snapshot_free (arg : * mut c_void) ; }
};
}
