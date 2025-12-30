// Generated macro for split_content (function)
macro_rules! Depcrate_unix_linux_processsplit_content {
() => {
// Module: crate::unix::linux::process
// Provides: {"split_content"}
// Dependencies: {}
fn split_content (mut data : & [u8]) -> Vec < OsString > { let mut out = Vec :: with_capacity (10) ; while let Some (pos) = data . iter () . position (| c | * c == 0) { let s = & data [.. pos] . trim_ascii () ; if ! s . is_empty () { out . push (OsStr :: from_bytes (s) . to_os_string ()) ; } data = & data [pos + 1 ..] ; } if ! data . is_empty () { let s = data . trim_ascii () ; if ! s . is_empty () { out . push (OsStr :: from_bytes (s) . to_os_string ()) ; } } out }
};
}
