// Generated macro for from_zero_terminated (function)
macro_rules! Depcrate_windows_diskfrom_zero_terminated {
() => {
// Module: crate::windows::disk
// Provides: {"from_zero_terminated"}
// Dependencies: {}
# [doc = " Creates a copy of the first zero-terminated wide string in `buf`."] # [doc = " The copy includes the zero terminator."] fn from_zero_terminated (buf : & [u16]) -> Vec < u16 > { let end = buf . iter () . position (| & x | x == 0) . unwrap_or (buf . len ()) ; buf [..= end] . to_vec () }
};
}
