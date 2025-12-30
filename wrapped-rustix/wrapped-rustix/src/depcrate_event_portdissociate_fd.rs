// Generated macro for dissociate_fd (function)
macro_rules! Depcrate_event_portdissociate_fd {
() => {
// Module: crate::event::port
// Provides: {"dissociate_fd"}
// Dependencies: {}
# [doc = " `port_dissociate(_, PORT_SOURCE_FD, _)`—Dissociates a file descriptor"] # [doc = " from a port."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The file descriptor passed into this function must have been previously"] # [doc = " associated with the port via [`associate_fd`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [OpenSolaris]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_dissociate"] # [doc = " [illumos]: https://illumos.org/man/3C/port_dissociate"] # [doc (alias = "port_dissociate")] pub unsafe fn dissociate_fd < Fd : AsFd , RawFd : AsRawFd > (port : Fd , object : RawFd) -> io :: Result < () > { syscalls :: port_dissociate (port . as_fd () , c :: PORT_SOURCE_FD , object . as_raw_fd () as _) }
};
}
