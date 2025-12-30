// Generated macro for impl_107 (impl)
macro_rules! Depcrate_listenerimpl_107 {
() => {
// Module: crate::listener
// Provides: {"impl_107"}
// Dependencies: {}
impl Listener { # [doc = " Accept a queued connection from this listener."] pub fn accept (& mut self) -> Result < Channel , Error > { let sess = self . sess . lock () ; unsafe { let chan = raw :: libssh2_channel_forward_accept (self . raw) ; let err = sess . last_error () ; Channel :: from_raw_opt (chan , err , & self . sess) } } pub (crate) fn from_raw_opt (raw : * mut raw :: LIBSSH2_LISTENER , err : Option < Error > , sess : & Arc < Mutex < SessionInner > > ,) -> Result < Self , Error > { if raw . is_null () { Err (err . unwrap_or_else (Error :: unknown)) } else { Ok (Self { raw , sess : Arc :: clone (sess) , }) } } }
};
}
