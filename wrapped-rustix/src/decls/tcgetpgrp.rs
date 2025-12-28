macro_rules! deps {
    () => {
        Pid!();
        Result!();
    };
}

macro_rules! tcgetpgrp {
    () => {
        deps!();
        # [doc = " `tcgetpgrp(fd)`—Get the terminal foreground process group."] # [doc = ""] # [doc = " Also known as the `TIOCGPGRP` operation with `ioctl`."] # [doc = ""] # [doc = " On Linux, if `fd` is a pseudo-terminal, the underlying system call here can"] # [doc = " return a pid of 0, which rustix's `Pid` type doesn't support. So rustix"] # [doc = " instead handles this case by failing with [`io::Errno::OPNOTSUPP`] if the"] # [doc = " pid is 0."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/tcgetpgrp.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/tcgetpgrp.3.html"] # [cfg (not (any (windows , target_os = "wasi")))] # [inline] # [doc (alias = "TIOCGPGRP")] pub fn tcgetpgrp < Fd : AsFd > (fd : Fd) -> io :: Result < Pid > { backend :: termios :: syscalls :: tcgetpgrp (fd . as_fd ()) }
    };
}

tcgetpgrp!();