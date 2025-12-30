// Generated macro for get_exe_and_name_backup (function)
macro_rules! Depcrate_unix_apple_macos_processget_exe_and_name_backup {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"get_exe_and_name_backup"}
// Dependencies: {}
# [doc = " Less efficient way to retrieve `exe` and `name`."] unsafe fn get_exe_and_name_backup (process : & mut ProcessInner , refresh_kind : ProcessRefreshKind , force_check : bool ,) -> bool { let exe_needs_update = refresh_kind . exe () . needs_update (| | process . exe . is_none ()) ; if ! process . name . is_empty () && ! exe_needs_update && ! force_check { return true ; } let mut buffer : Vec < u8 > = Vec :: with_capacity (libc :: PROC_PIDPATHINFO_MAXSIZE as _) ; unsafe { match libc :: proc_pidpath (process . pid . 0 , buffer . as_mut_ptr () as * mut _ , libc :: PROC_PIDPATHINFO_MAXSIZE as _ ,) { x if x > 0 => { buffer . set_len (x as _) ; let tmp = OsString :: from_vec (buffer) ; let exe = PathBuf :: from (tmp) ; if process . name . is_empty () { exe . file_name () . unwrap_or_default () . clone_into (& mut process . name) ; } if exe_needs_update { process . exe = Some (exe) ; } true } _ => false , } } }
};
}
