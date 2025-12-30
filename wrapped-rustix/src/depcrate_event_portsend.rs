// Generated macro for send (function)
macro_rules! Depcrate_event_portsend {
() => {
// Module: crate::event::port
// Provides: {"send"}
// Dependencies: {}
# [doc = " `port_send(port, events, userdata)`—Sends an event to a port."] # [doc = ""] # [doc = " # References"] # [doc = "  - [OpenSolaris]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_send/"] # [doc = " [illumos]: https://illumos.org/man/3C/port_send"] # [doc (alias = "port_send")] pub fn send < Fd : AsFd > (port : Fd , events : i32 , userdata : * mut ffi :: c_void) -> io :: Result < () > { syscalls :: port_send (port . as_fd () , events , userdata . cast ()) }
};
}
