macro_rules! deps {
    () => {
        Pid!();
        Result!();
    };
}

macro_rules! tcsetpgrp {
    () => {
        deps!();
        # [doc = " `tcsetpgrp(fd, pid)`—Set the terminal foreground process group."] # [doc = ""] # [doc = " Also known as the `TIOCSPGRP` operation with `ioctl`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/tcsetpgrp.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/tcsetpgrp.3.html"] # [cfg (not (any (windows , target_os = "wasi")))] # [inline] # [doc (alias = "TIOCSPGRP")] pub fn tcsetpgrp < Fd : AsFd > (fd : Fd , pid : Pid) -> io :: Result < () > { backend :: termios :: syscalls :: tcsetpgrp (fd . as_fd () , pid) }
    };
}

tcsetpgrp!();