// Generated macro for kevent_timespec (function)
macro_rules! Depcrate_event_kqueuekevent_timespec {
() => {
// Module: crate::event::kqueue
// Provides: {"kevent_timespec"}
// Dependencies: {}
# [doc = " `kevent(kqueue, changelist, eventlist, timeout)`—Wait for events on a"] # [doc = " `kqueue`."] # [doc = ""] # [doc = " If an unsupported timeout is passed, this function fails with"] # [doc = " [`io::Errno::INVAL`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The file descriptors referred to by the `Event` structs must be valid for"] # [doc = " the lifetime of the `kqueue` file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kevent.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=kevent&sektion=2"] # [doc = " [OpenBSD]: https://man.openbsd.org/kevent.2"] # [doc = " [NetBSD]: https://man.netbsd.org/kevent.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=kevent&section=2"] pub unsafe fn kevent_timespec < Fd : AsFd , Buf : Buffer < Event > > (kqueue : Fd , changelist : & [Event] , mut eventlist : Buf , timeout : Option < & Timespec > ,) -> io :: Result < Buf :: Output > { let len = syscalls :: kevent (kqueue . as_fd () , changelist , eventlist . parts_mut () , timeout) . map (| res | res as _) ? ; Ok (eventlist . assume_init (len)) }
};
}
