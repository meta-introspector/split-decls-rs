// Generated macro for SessionInner (struct)
macro_rules! Depcrate_sessionSessionInner {
() => {
// Module: crate::session
// Provides: {"SessionInner"}
// Dependencies: {}
pub (crate) struct SessionInner { pub (crate) raw : * mut raw :: LIBSSH2_SESSION , # [cfg (unix)] tcp : Option < Box < dyn AsRawFd > > , # [cfg (windows)] tcp : Option < Box < dyn AsRawSocket > > , }
};
}
