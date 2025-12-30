// Generated macro for getn (function)
macro_rules! Depcrate_event_portgetn {
() => {
// Module: crate::event::port
// Provides: {"getn"}
// Dependencies: {}
# [doc = " `port_getn(port, events, min_events, timeout)`—Gets multiple events from"] # [doc = " a port."] # [doc = ""] # [doc = " If `events` is empty, this does nothing and returns immediately."] # [doc = ""] # [doc = " To query the number of events without retrieving any, use [`getn_query`]."] # [doc = ""] # [doc = " If an unsupported timeout is passed, this function fails with"] # [doc = " [`io::Errno::INVAL`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [OpenSolaris]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_getn/"] # [doc = " [illumos]: https://illumos.org/man/3C/port_getn"] # [doc (alias = "port_getn")] pub fn getn < Fd : AsFd , Buf : Buffer < Event > > (port : Fd , mut events : Buf , min_events : u32 , timeout : Option < & Timespec > ,) -> io :: Result < Buf :: Output > { let nevents = unsafe { syscalls :: port_getn (port . as_fd () , events . parts_mut () , min_events , timeout) ? } ; unsafe { Ok (events . assume_init (nevents)) } }
};
}
