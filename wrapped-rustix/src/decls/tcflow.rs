macro_rules! deps {
    () => {
        Action!();
        Result!();
    };
}

macro_rules! tcflow {
    () => {
        deps!();
        # [doc = " `tcflow(fd, action)`—Suspend or resume transmission or reception."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX `tcflow`]"] # [doc = "  - [Linux `ioctl_tty`]"] # [doc = "  - [Linux `termios`]"] # [doc = ""] # [doc = " [POSIX `tcflow`]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/tcflow.html"] # [doc = " [Linux `ioctl_tty`]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [doc = " [Linux `termios`]: https://man7.org/linux/man-pages/man3/termios.3.html"] # [cfg (not (target_os = "espidf"))] # [inline] # [doc (alias = "TCXONC")] pub fn tcflow < Fd : AsFd > (fd : Fd , action : Action) -> io :: Result < () > { backend :: termios :: syscalls :: tcflow (fd . as_fd () , action) }
    };
}

tcflow!()