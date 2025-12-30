// Generated macro for add (function)
macro_rules! Depcrate_event_epolladd {
() => {
// Module: crate::event::epoll
// Provides: {"add"}
// Dependencies: {}
# [doc = " `epoll_ctl(self, EPOLL_CTL_ADD, data, event)`—Adds an element to an epoll"] # [doc = " object."] # [doc = ""] # [doc = " This registers interest in any of the events set in `event_flags` occurring"] # [doc = " on the file descriptor associated with `data`."] # [doc = ""] # [doc = " `close`ing a file descriptor does not necessarily unregister interest which"] # [doc = " can lead to spurious events being returned from [`epoll::wait`]. If a file"] # [doc = " descriptor is an `Arc<dyn SystemResource>`, then `epoll` can be thought to"] # [doc = " maintain a `Weak<dyn SystemResource>` to the file descriptor. Check the"] # [doc = " [faq] for details."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/epoll_ctl.2.html"] # [doc = " [illumos]: https://www.illumos.org/man/3C/epoll_ctl"] # [doc = " [faq]: https://man7.org/linux/man-pages/man7/epoll.7.html#:~:text=Will%20closing%20a%20file%20descriptor%20cause%20it%20to%20be%20removed%20from%20all%0A%20%20%20%20%20%20%20%20%20%20epoll%20interest%20lists%3F"] # [doc (alias = "epoll_ctl")] # [inline] pub fn add < EpollFd : AsFd , SourceFd : AsFd > (epoll : EpollFd , source : SourceFd , data : epoll :: EventData , event_flags : epoll :: EventFlags ,) -> io :: Result < () > { syscalls :: epoll_add (epoll . as_fd () , source . as_fd () , & Event { flags : event_flags , data , # [cfg (all (libc , target_os = "redox"))] _pad : 0 , } ,) }
};
}
