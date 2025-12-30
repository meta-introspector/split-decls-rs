// Generated macro for read_u64 (function)
macro_rules! Depcrate_unix_linux_systemread_u64 {
() => {
// Module: crate::unix::linux::system
// Provides: {"read_u64"}
// Dependencies: {}
fn read_u64 (filename : & str) -> Option < u64 > { let result = get_all_utf8_data (filename , 16_635) . ok () . and_then (| d | u64 :: from_str (d . trim ()) . ok ()) ; if result . is_none () { sysinfo_debug ! ("Failed to read u64 in filename {}" , filename) ; } result }
};
}
