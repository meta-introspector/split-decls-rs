// Generated macro for get_bsd_name (function)
macro_rules! Depcrate_unix_apple_diskget_bsd_name {
() => {
// Module: crate::unix::apple::disk
// Provides: {"get_bsd_name"}
// Dependencies: {}
# [cfg (target_os = "macos")] fn get_bsd_name (disk : & libc :: statfs) -> Option < Vec < u8 > > { unsafe { CStr :: from_ptr (disk . f_mntfromname . as_ptr ()) . to_bytes_with_nul () . strip_prefix (b"/dev/") . map (| slice | slice . to_vec ()) . or_else (| | { sysinfo_debug ! ("unknown disk mount path format") ; None }) } }
};
}
