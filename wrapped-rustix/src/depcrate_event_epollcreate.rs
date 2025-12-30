// Generated macro for create (function)
macro_rules! Depcrate_event_epollcreate {
() => {
// Module: crate::event::epoll
// Provides: {"create"}
// Dependencies: {}
# [doc = " `epoll_create1(flags)`—Creates a new epoll object."] # [doc = ""] # [doc = " Use the [`epoll::CreateFlags::CLOEXEC`] flag to prevent the resulting file"] # [doc = " descriptor from being implicitly passed across `exec` boundaries."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/epoll_create.2.html"] # [doc = " [illumos]: https://www.illumos.org/man/3C/epoll_create"] # [inline] # [doc (alias = "epoll_create1")] pub fn create (flags : epoll :: CreateFlags) -> io :: Result < OwnedFd > { syscalls :: epoll_create (flags) }
};
}
