macro_rules! deps {
    () => {
        Result!();
        Tiocgptpeer!();
    };
}

macro_rules! ioctl_tiocgptpeer {
    () => {
        deps!();
        # [doc = " `ioctl(fd, TIOCGPTPEER)`—Open the user side of a pseudoterminal."] # [doc = ""] # [doc = " This function is currently only implemented on Linux."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/ioctl_tty.2.html"] # [cfg (target_os = "linux")] # [inline] pub fn ioctl_tiocgptpeer < Fd : AsFd > (fd : Fd , flags : OpenptFlags) -> io :: Result < OwnedFd > { unsafe { ioctl :: ioctl (fd , Tiocgptpeer (flags)) } }
    };
}

ioctl_tiocgptpeer!();