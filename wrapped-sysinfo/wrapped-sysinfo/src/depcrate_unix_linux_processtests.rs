// Generated macro for tests (module)
macro_rules! Depcrate_unix_linux_processtests {
() => {
// Module: crate::unix::linux::process
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: split_content ; use std :: ffi :: OsString ; # [test] fn test_copy_file () { assert_eq ! (split_content (b"hello\0") , vec ! [OsString :: from ("hello")]) ; assert_eq ! (split_content (b"hello") , vec ! [OsString :: from ("hello")]) ; assert_eq ! (split_content (b"hello\0b") , vec ! [OsString :: from ("hello") , "b" . into ()]) ; assert_eq ! (split_content (b"hello\0\0\0\0b") , vec ! [OsString :: from ("hello") , "b" . into ()]) ; } }
};
}
