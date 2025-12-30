// Generated macro for endswith (function)
macro_rules! Depcrate_unix_usersendswith {
() => {
// Module: crate::unix::users
// Provides: {"endswith"}
// Dependencies: {}
fn endswith (s1 : * const std :: ffi :: c_char , s2 : & [u8]) -> bool { if s1 . is_null () { return false ; } unsafe { let mut len = libc :: strlen (s1) as isize - 1 ; let mut i = s2 . len () as isize - 1 ; while len >= 0 && i >= 0 && * s1 . offset (len) == s2 [i as usize] as std :: ffi :: c_char { i -= 1 ; len -= 1 ; } i == - 1 } }
};
}
