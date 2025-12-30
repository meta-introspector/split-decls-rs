// Generated macro for retry_eintr (macro)
macro_rules! Depcrate_macrosretry_eintr {
() => {
// Module: crate::macros
// Provides: {"retry_eintr"}
// Dependencies: {}
# [cfg (all (unix , not (feature = "unknown-ci")))] # [allow (unused_macros)] macro_rules ! retry_eintr { (set_to_0 => $ ($ t : tt) +) => { { # [allow (unused_unsafe)] let errno = unsafe { crate :: unix :: libc_errno () } ; if ! errno . is_null () { # [allow (unused_unsafe)] unsafe { * errno = 0 ; } } retry_eintr ! ($ ($ t) +) } } ; ($ errno_value : ident => $ ($ t : tt) +) => { { loop { let ret = $ ($ t) +; if ret < 0 { let tmp = std :: io :: Error :: last_os_error () ; if tmp . kind () == std :: io :: ErrorKind :: Interrupted { continue ; } $ errno_value = tmp . raw_os_error () . unwrap_or (0) ; } break ret ; } } } ; ($ ($ t : tt) +) => { { loop { let ret = $ ($ t) +; if ret < 0 && std :: io :: Error :: last_os_error () . kind () == std :: io :: ErrorKind :: Interrupted { continue ; } break ret ; } } } ; }
};
}
