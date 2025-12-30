// Generated macro for append_to_string (function)
macro_rules! Depcrate_ioappend_to_string {
() => {
// Module: crate::io
// Provides: {"append_to_string"}
// Dependencies: {}
pub (crate) unsafe fn append_to_string < F > (buf : & mut String , f : F) -> Result < usize > where F : FnOnce (& mut Vec < u8 >) -> Result < usize > , { let mut g = Guard { len : buf . len () , buf : unsafe { buf . as_mut_vec () } } ; let ret = f (g . buf) ; let appended = unsafe { g . buf . get_unchecked (g . len ..) } ; if str :: from_utf8 (appended) . is_err () { ret . and_then (| _ | Err (Error :: INVALID_UTF8)) } else { g . len = g . buf . len () ; ret } }
};
}
