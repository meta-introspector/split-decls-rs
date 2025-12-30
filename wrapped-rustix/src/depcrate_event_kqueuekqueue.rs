// Generated macro for kqueue (function)
macro_rules! Depcrate_event_kqueuekqueue {
() => {
// Module: crate::event::kqueue
// Provides: {"kqueue"}
// Dependencies: {}
# [doc = " `kqueue()`—Create a new `kqueue` file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kqueue.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=kqueue&sektion=2"] # [doc = " [OpenBSD]: https://man.openbsd.org/kqueue.2"] # [doc = " [NetBSD]: https://man.netbsd.org/kqueue.2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=kqueue&section=2"] pub fn kqueue () -> io :: Result < OwnedFd > { syscalls :: kqueue () }
};
}
