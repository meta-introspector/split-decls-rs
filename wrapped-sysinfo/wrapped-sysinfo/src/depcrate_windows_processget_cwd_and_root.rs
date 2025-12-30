// Generated macro for get_cwd_and_root (function)
macro_rules! Depcrate_windows_processget_cwd_and_root {
() => {
// Module: crate::windows::process
// Provides: {"get_cwd_and_root"}
// Dependencies: {}
fn get_cwd_and_root < T : RtlUserProcessParameters > (params : & T , handle : HANDLE , refresh_kind : ProcessRefreshKind , cwd : & mut Option < PathBuf > , root : & mut Option < PathBuf > ,) { let cwd_needs_update = refresh_kind . cwd () . needs_update (| | cwd . is_none ()) ; let root_needs_update = refresh_kind . root () . needs_update (| | root . is_none ()) ; if ! cwd_needs_update && ! root_needs_update { return ; } match params . get_cwd (handle) { Ok (buffer) => unsafe { let tmp_cwd = PathBuf :: from (null_terminated_wchar_to_string (buffer . as_slice ())) ; update_root (refresh_kind , & tmp_cwd , root) ; if cwd_needs_update { * cwd = Some (tmp_cwd) ; } } , Err (_e) => { sysinfo_debug ! ("get_cwd_and_root failed to get data: {:?}" , _e) ; * cwd = None ; } } }
};
}
