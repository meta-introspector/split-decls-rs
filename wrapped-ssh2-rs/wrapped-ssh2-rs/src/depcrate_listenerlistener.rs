// Generated macro for Listener (struct)
macro_rules! Depcrate_listenerListener {
() => {
// Module: crate::listener
// Provides: {"Listener"}
// Dependencies: {}
# [doc = " A listener represents a forwarding port from the remote server."] # [doc = ""] # [doc = " New channels can be accepted from a listener which represent connections on"] # [doc = " the remote server's port."] pub struct Listener { raw : * mut raw :: LIBSSH2_LISTENER , sess : Arc < Mutex < SessionInner > > , }
};
}
