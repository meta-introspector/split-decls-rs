// Generated macro for osstr2str (function)
macro_rules! Depcrate_os_wasi_fsosstr2str {
() => {
// Module: crate::os::wasi::fs
// Provides: {"osstr2str"}
// Dependencies: {}
fn osstr2str (f : & OsStr) -> io :: Result < & str > { f . to_str () . ok_or_else (| | io :: const_error ! (io :: ErrorKind :: Uncategorized , "input must be utf-8")) }
};
}
