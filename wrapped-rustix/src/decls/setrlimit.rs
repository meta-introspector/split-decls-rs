macro_rules! deps {
    () => {
        Result!();
        Rlimit!();
    };
}

macro_rules! setrlimit {
    () => {
        deps!();
        # [doc = " `setrlimit(resource, new)`—Set a process resource limit value."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/setrlimit.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setrlimit.2.html"] # [inline] pub fn setrlimit (resource : Resource , new : Rlimit) -> io :: Result < () > { backend :: process :: syscalls :: setrlimit (resource , new) }
    };
}

setrlimit!();