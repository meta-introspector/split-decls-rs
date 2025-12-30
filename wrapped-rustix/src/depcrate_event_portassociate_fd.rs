// Generated macro for associate_fd (function)
macro_rules! Depcrate_event_portassociate_fd {
() => {
// Module: crate::event::port
// Provides: {"associate_fd"}
// Dependencies: {}
# [doc = " `port_associate(_, PORT_SOURCE_FD, _, _, _)`—Associates a file descriptor"] # [doc = " with a port."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Any `object`s passed into the `port` must be valid for the lifetime of the"] # [doc = " `port`. Logically, `port` keeps a borrowed reference to the `object` until"] # [doc = " it is removed via [`dissociate_fd`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [OpenSolaris]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_associate/"] # [doc = " [illumos]: https://illumos.org/man/3C/port_associate"] # [doc (alias = "port_associate")] pub unsafe fn associate_fd < Fd : AsFd , RawFd : AsRawFd > (port : Fd , object : RawFd , events : PollFlags , userdata : * mut ffi :: c_void ,) -> io :: Result < () > { syscalls :: port_associate (port . as_fd () , c :: PORT_SOURCE_FD , object . as_raw_fd () as _ , events . bits () as _ , userdata . cast () ,) }
};
}
