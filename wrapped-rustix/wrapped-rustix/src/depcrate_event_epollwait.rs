// Generated macro for wait (function)
macro_rules! Depcrate_event_epollwait {
() => {
// Module: crate::event::epoll
// Provides: {"wait"}
// Dependencies: {}
# [doc = " `epoll_wait(self, events, timeout)`—Waits for registered events of"] # [doc = " interest."] # [doc = ""] # [doc = " For each event of interest, an element is written to `events`."] # [doc = ""] # [doc = " Linux versions older than 5.11 (those that don't support `epoll_pwait2`)"] # [doc = " don't support timeouts greater than `c_int::MAX` milliseconds; if an"] # [doc = " unsupported timeout is passed, this function fails with"] # [doc = " [`io::Errno::INVAL`]. Enable the \"linux_5_11\" feature to enable the full"] # [doc = " range of timeouts."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/epoll_wait.2.html"] # [doc = " [illumos]: https://www.illumos.org/man/3C/epoll_wait"] # [doc (alias = "epoll_wait")] # [inline] pub fn wait < EpollFd : AsFd , Buf : Buffer < Event > > (epoll : EpollFd , mut event_list : Buf , timeout : Option < & Timespec > ,) -> io :: Result < Buf :: Output > { let nfds = unsafe { syscalls :: epoll_wait (epoll . as_fd () , event_list . parts_mut () , timeout) ? } ; unsafe { Ok (event_list . assume_init (nfds)) } }
};
}
