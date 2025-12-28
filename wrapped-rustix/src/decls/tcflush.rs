macro_rules! deps {
    () => {
        Result!();
        QueueSelector!();
    };
}

macro_rules! tcflush {
    () => {
        deps!();
        # [doc = " `tcflush(fd, queue_selector)`—Wait until all pending output has been"] # [doc = " written."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX `tcflush`]"] # [doc = "  - [Linux `ioctl_tty`]"] # [doc = "  - [Linux `termios`]"] # [doc = ""] # [doc = " [POSIX `tcflush`]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/tcflush.html"] # [doc = " [Linux `ioctl_tty`]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [doc = " [Linux `termios`]: https://man7.org/linux/man-pages/man3/termios.3.html"] # [cfg (not (target_os = "espidf"))] # [inline] # [doc (alias = "TCFLSH")] pub fn tcflush < Fd : AsFd > (fd : Fd , queue_selector : QueueSelector) -> io :: Result < () > { backend :: termios :: syscalls :: tcflush (fd . as_fd () , queue_selector) }
    };
}

tcflush!();