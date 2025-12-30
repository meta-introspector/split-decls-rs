// Generated macro for get_tgid (function)
macro_rules! Depcrate_unix_linux_processget_tgid {
() => {
// Module: crate::unix::linux::process
// Provides: {"get_tgid"}
// Dependencies: {}
fn get_tgid (file_path : & Path) -> Option < Pid > { const TGID_KEY : & str = "Tgid:" ; let status_data = get_all_utf8_data (file_path , 16_385) . ok () ? ; let tgid_line = status_data . lines () . find (| line | line . starts_with (TGID_KEY)) ? ; tgid_line [TGID_KEY . len () ..] . trim_start () . parse () . ok () }
};
}
