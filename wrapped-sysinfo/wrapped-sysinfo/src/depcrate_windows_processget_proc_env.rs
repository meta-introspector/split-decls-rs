// Generated macro for get_proc_env (function)
macro_rules! Depcrate_windows_processget_proc_env {
() => {
// Module: crate::windows::process
// Provides: {"get_proc_env"}
// Dependencies: {}
fn get_proc_env < T : RtlUserProcessParameters > (params : & T , handle : HANDLE , refresh_kind : ProcessRefreshKind , environ : & mut Vec < OsString > ,) { if ! refresh_kind . environ () . needs_update (| | environ . is_empty ()) { return ; } match params . get_environ (handle) { Ok (buffer) => { let equals = "=" . encode_utf16 () . next () . unwrap () ; let raw_env = buffer ; environ . clear () ; let mut begin = 0 ; while let Some (offset) = raw_env [begin ..] . iter () . position (| & c | c == 0) { let end = begin + offset ; if raw_env [begin .. end] . contains (& equals) { environ . push (OsString :: from_wide (& raw_env [begin .. end])) ; begin = end + 1 ; } else { break ; } } } Err (_e) => { sysinfo_debug ! ("get_proc_env failed to get data: {}" , _e) ; * environ = Vec :: new () ; } } }
};
}
