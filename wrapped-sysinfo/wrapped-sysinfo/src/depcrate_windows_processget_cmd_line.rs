// Generated macro for get_cmd_line (function)
macro_rules! Depcrate_windows_processget_cmd_line {
() => {
// Module: crate::windows::process
// Provides: {"get_cmd_line"}
// Dependencies: {}
fn get_cmd_line < T : RtlUserProcessParameters > (params : & T , handle : HANDLE , refresh_kind : ProcessRefreshKind , cmd_line : & mut Vec < OsString > ,) { if ! refresh_kind . cmd () . needs_update (| | cmd_line . is_empty ()) { return ; } if * windows_8_1_or_newer () { * cmd_line = get_cmd_line_new (handle) ; } else { * cmd_line = get_cmd_line_old (params , handle) ; } }
};
}
