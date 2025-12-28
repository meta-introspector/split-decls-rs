macro_rules! deps {
    () => {
        Termios!();
        Result!();
    };
}

macro_rules! tcgetattr {
    () => {
        deps!();
        # [doc = " `tcgetattr(fd)`—Get terminal attributes."] # [doc = ""] # [doc = " Also known as the `TCGETS` (or `TCGETS2` on Linux) operation with `ioctl`."] # [doc = ""] # [doc = " On Linux, this uses `TCGETS2`. If that fails in a way that indicates that"] # [doc = " the host doesn't support it, this falls back to the old `TCGETS`, manually"] # [doc = " initializes the fields that `TCGETS` doesn't initialize, and fails with"] # [doc = " `io::Errno::RANGE` if the input or output speeds cannot be supported."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX `tcgetattr`]"] # [doc = "  - [Linux `ioctl_tty`]"] # [doc = "  - [Linux `termios`]"] # [doc = ""] # [doc = " [POSIX `tcgetattr`]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/tcgetattr.html"] # [doc = " [Linux `ioctl_tty`]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [doc = " [Linux `termios`]: https://man7.org/linux/man-pages/man3/termios.3.html"] # [cfg (not (any (windows , target_os = "espidf" , target_os = "wasi")))] # [inline] # [doc (alias = "TCGETS")] # [doc (alias = "TCGETS2")] # [doc (alias = "tcgetattr2")] pub fn tcgetattr < Fd : AsFd > (fd : Fd) -> io :: Result < Termios > { backend :: termios :: syscalls :: tcgetattr (fd . as_fd ()) }
    };
}

tcgetattr!()