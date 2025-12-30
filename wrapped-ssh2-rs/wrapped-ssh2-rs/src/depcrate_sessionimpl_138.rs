// Generated macro for impl_138 (impl)
macro_rules! Depcrate_sessionimpl_138 {
() => {
// Module: crate::session
// Provides: {"impl_138"}
// Dependencies: {}
impl SessionInner { # [doc = " Translate a return code into a Rust-`Result`."] pub fn rc (& self , rc : c_int) -> Result < () , Error > { if rc >= 0 { Ok (()) } else { Err (Error :: from_session_error_raw (self . raw , rc)) } } pub fn last_error (& self) -> Option < Error > { Error :: last_session_error_raw (self . raw) } # [doc = " Set or clear blocking mode on session"] pub fn set_blocking (& self , blocking : bool) { unsafe { raw :: libssh2_session_set_blocking (self . raw , blocking as c_int) } } # [doc = " Returns whether the session was previously set to nonblocking."] pub fn is_blocking (& self) -> bool { unsafe { raw :: libssh2_session_get_blocking (self . raw) != 0 } } }
};
}
