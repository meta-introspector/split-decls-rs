macro_rules! deps {
    () => {
        Winsize!();
        Result!();
    };
}

macro_rules! tcgetwinsize {
    () => {
        deps!();
        # [doc = " `tcgetwinsize(fd)`—Get the current terminal window size."] # [doc = ""] # [doc = " Also known as the `TIOCGWINSZ` operation with `ioctl`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html"] # [cfg (not (any (windows , target_os = "horizon" , target_os = "espidf" , target_os = "wasi")))] # [inline] # [doc (alias = "TIOCGWINSZ")] pub fn tcgetwinsize < Fd : AsFd > (fd : Fd) -> io :: Result < Winsize > { backend :: termios :: syscalls :: tcgetwinsize (fd . as_fd ()) }
    };
}

tcgetwinsize!()