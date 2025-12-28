macro_rules! deps {
    () => {
        Result!();
        NoArg!();
    };
}

macro_rules! ioctl_tiocnxcl {
    () => {
        deps!();
        # [doc = " `ioctl(fd, TIOCNXCL)`—Disables exclusive mode on a terminal."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=tty&sektion=4"] # [doc = " [NetBSD]: https://man.netbsd.org/tty.4"] # [doc = " [OpenBSD]: https://man.openbsd.org/tty.4"] # [cfg (not (any (windows , target_os = "horizon" , target_os = "redox" , target_os = "wasi")))] # [inline] # [doc (alias = "TIOCNXCL")] pub fn ioctl_tiocnxcl < Fd : AsFd > (fd : Fd) -> io :: Result < () > { unsafe { let ctl = ioctl :: NoArg :: < { c :: TIOCNXCL as _ } > :: new () ; ioctl :: ioctl (fd , ctl) } }
    };
}

ioctl_tiocnxcl!()