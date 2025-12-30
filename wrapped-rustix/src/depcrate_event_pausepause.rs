// Generated macro for pause (function)
macro_rules! Depcrate_event_pausepause {
() => {
// Module: crate::event::pause
// Provides: {"pause"}
// Dependencies: {}
# [doc = " `pause()`—Sleep until interrupted by a signal."] # [doc = ""] # [doc = " The POSIX `pause` interface returns an error code, but the only thing"] # [doc = " `pause` does is sleep until interrupted by a signal. If it were exposed in"] # [doc = " the API here it would always return `Errno::INTR`, so for simplicity the"] # [doc = " return value is omitted."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/pause.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pause.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/pause.3.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=pause&sektion=3"] # [doc = " [NetBSD]: https://man.netbsd.org/pause.3"] # [doc = " [OpenBSD]: https://man.openbsd.org/pause.3"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=pause&section=3"] # [doc = " [illumos]: https://illumos.org/man/2/pause"] # [inline] pub fn pause () { backend :: event :: syscalls :: pause () }
};
}
