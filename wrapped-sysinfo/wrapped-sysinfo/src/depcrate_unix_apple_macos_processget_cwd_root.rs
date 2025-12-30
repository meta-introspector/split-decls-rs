// Generated macro for get_cwd_root (function)
macro_rules! Depcrate_unix_apple_macos_processget_cwd_root {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"get_cwd_root"}
// Dependencies: {}
unsafe fn get_cwd_root (process : & mut ProcessInner , refresh_kind : ProcessRefreshKind) { let cwd_needs_update = refresh_kind . cwd () . needs_update (| | process . cwd . is_none ()) ; let root_needs_update = refresh_kind . root () . needs_update (| | process . root . is_none ()) ; if ! cwd_needs_update && ! root_needs_update { return ; } unsafe { let mut vnodepathinfo = mem :: zeroed :: < libc :: proc_vnodepathinfo > () ; let result = libc :: proc_pidinfo (process . pid . 0 , libc :: PROC_PIDVNODEPATHINFO , 0 , & mut vnodepathinfo as * mut _ as * mut _ , mem :: size_of :: < libc :: proc_vnodepathinfo > () as _ ,) ; if result < 1 { sysinfo_debug ! ("Failed to retrieve cwd and root for {}" , process . pid . 0) ; return ; } if cwd_needs_update { process . cwd = convert_node_path_info (& vnodepathinfo . pvi_cdir) ; } if root_needs_update { process . root = convert_node_path_info (& vnodepathinfo . pvi_rdir) ; } } }
};
}
