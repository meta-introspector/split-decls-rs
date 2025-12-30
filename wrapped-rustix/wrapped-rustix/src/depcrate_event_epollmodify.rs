// Generated macro for modify (function)
macro_rules! Depcrate_event_epollmodify {
() => {
// Module: crate::event::epoll
// Provides: {"modify"}
// Dependencies: {}
# [doc = " `epoll_ctl(self, EPOLL_CTL_MOD, target, event)`—Modifies an element in a"] # [doc = " given epoll object."] # [doc = ""] # [doc = " This sets the events of interest with `target` to `events`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/epoll_ctl.2.html"] # [doc = " [illumos]: https://www.illumos.org/man/3C/epoll_ctl"] # [doc (alias = "epoll_ctl")] # [inline] pub fn modify < EpollFd : AsFd , SourceFd : AsFd > (epoll : EpollFd , source : SourceFd , data : epoll :: EventData , event_flags : epoll :: EventFlags ,) -> io :: Result < () > { syscalls :: epoll_mod (epoll . as_fd () , source . as_fd () , & Event { flags : event_flags , data , # [cfg (all (libc , target_os = "redox"))] _pad : 0 , } ,) }
};
}
