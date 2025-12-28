macro_rules! deps {
    () => {
        Result!();
        Tiocsctty!();
    };
}

macro_rules! ioctl_tiocsctty {
    () => {
        deps!();
        # [doc = " `ioctl(fd, TIOCSCTTY, 0)`—Sets the controlling terminal for the process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=tty&sektion=4"] # [doc = " [NetBSD]: https://man.netbsd.org/tty.4"] # [doc = " [OpenBSD]: https://man.openbsd.org/tty.4"] # [cfg (not (any (windows , target_os = "aix" , target_os = "horizon" , target_os = "redox" , target_os = "wasi")))] # [inline] # [doc (alias = "TIOCSCTTY")] pub fn ioctl_tiocsctty < Fd : AsFd > (fd : Fd) -> io :: Result < () > { unsafe { ioctl :: ioctl (fd , Tiocsctty) } }
    };
}

ioctl_tiocsctty!()