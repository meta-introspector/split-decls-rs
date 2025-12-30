// Generated macro for initialize_geom (function)
macro_rules! Depcrate_unix_freebsd_diskinitialize_geom {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"initialize_geom"}
// Dependencies: {}
unsafe fn initialize_geom () -> Result < () , () > { let version = unsafe { devstat_getversion (null_mut ()) } ; if version != 6 { sysinfo_debug ! ("version {version} of devstat is not supported") ; return Err (()) ; } let r = unsafe { geom_stats_open () } ; if r != 0 { sysinfo_debug ! ("`geom_stats_open` failed: {r}") ; Err (()) } else { Ok (()) } }
};
}
