// Generated macro for impl_791 (impl)
macro_rules! Depcrate_unix_linux_diskimpl_791 {
() => {
// Module: crate::unix::linux::disk
// Provides: {"impl_791"}
// Dependencies: {}
impl DiskStat { # [doc = " Returns the name and the values we're interested into."] fn new_from_line (line : & str) -> Option < (String , Self) > { let mut iter = line . split_whitespace () ; let name = iter . nth (2) . map (ToString :: to_string) ? ; let sectors_read = iter . nth (2) . and_then (| v | u64 :: from_str (v) . ok ()) . unwrap_or (0) ; let sectors_written = iter . nth (3) . and_then (| v | u64 :: from_str (v) . ok ()) . unwrap_or (0) ; Some ((name , Self { sectors_read , sectors_written , } ,)) } }
};
}
