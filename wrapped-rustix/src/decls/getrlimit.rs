macro_rules! deps {
    () => {
        Rlimit!();
    };
}

macro_rules! getrlimit {
    () => {
        deps!();
        # [doc = " `getrlimit(resource)`—Get a process resource limit value."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getrlimit.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getrlimit.2.html"] # [inline] pub fn getrlimit (resource : Resource) -> Rlimit { backend :: process :: syscalls :: getrlimit (resource) }
    };
}

getrlimit!()