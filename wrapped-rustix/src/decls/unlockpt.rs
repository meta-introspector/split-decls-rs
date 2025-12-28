macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! unlockpt {
    () => {
        deps!();
        # [doc = " `unlockpt(fd)`—Unlock a pseudoterminal."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/unlockpt.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/unlockpt.3.html"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Allocation.html#index-unlockpt"] # [inline] pub fn unlockpt < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: pty :: syscalls :: unlockpt (fd . as_fd ()) }
    };
}

unlockpt!()