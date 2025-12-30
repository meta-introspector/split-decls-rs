// Generated macro for eventfd (function)
macro_rules! Depcrate_event_eventfdeventfd {
() => {
// Module: crate::event::eventfd
// Provides: {"eventfd"}
// Dependencies: {}
# [doc = " `eventfd(initval, flags)`—Creates a file descriptor for event"] # [doc = " notification."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/eventfd.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?eventfd"] # [doc = " [illumos]: https://illumos.org/man/3C/eventfd"] # [inline] pub fn eventfd (initval : u32 , flags : EventfdFlags) -> io :: Result < OwnedFd > { backend :: event :: syscalls :: eventfd (initval , flags) }
};
}
