macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! tcsendbreak {
    () => {
        deps!();
        # [doc = " `tcsendbreak(fd, 0)`—Transmit zero-valued bits."] # [doc = ""] # [doc = " This transmits zero-valued bits for at least 0.25 seconds."] # [doc = ""] # [doc = " This function does not have a `duration` parameter, and always uses the"] # [doc = " implementation-defined value, which transmits for at least 0.25 seconds."] # [doc = ""] # [doc = " Also known as the `TCSBRK` operation with `ioctl`, with a duration"] # [doc = " parameter of 0."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX `tcsendbreak`]"] # [doc = "  - [Linux `ioctl_tty`]"] # [doc = "  - [Linux `termios`]"] # [doc = ""] # [doc = " [POSIX `tcsendbreak`]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/tcsendbreak.html"] # [doc = " [Linux `ioctl_tty`]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [doc = " [Linux `termios`]: https://man7.org/linux/man-pages/man3/termios.3.html"] # [inline] # [doc (alias = "TCSBRK")] pub fn tcsendbreak < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: termios :: syscalls :: tcsendbreak (fd . as_fd ()) }
    };
}

tcsendbreak!();