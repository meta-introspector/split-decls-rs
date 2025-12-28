macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! delete {
    () => {
        deps!();
        # [doc = " `epoll_ctl(self, EPOLL_CTL_DEL, target, NULL)`—Removes an element in a"] # [doc = " given epoll object."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/epoll_ctl.2.html"] # [doc = " [illumos]: https://www.illumos.org/man/3C/epoll_ctl"] # [doc (alias = "epoll_ctl")] # [inline] pub fn delete < EpollFd : AsFd , SourceFd : AsFd > (epoll : EpollFd , source : SourceFd) -> io :: Result < () > { syscalls :: epoll_del (epoll . as_fd () , source . as_fd ()) }
    };
}

delete!()