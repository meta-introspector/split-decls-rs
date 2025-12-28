macro_rules! deps {
    () => {
        Timespec!();
        Result!();
        DynamicClockId!();
    };
}

macro_rules! clock_gettime_dynamic {
    () => {
        deps!();
        # [doc = " Like [`clock_gettime`] but with support for dynamic clocks."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_gettime.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/clock_gettime.2.html"] # [cfg (not (target_os = "wasi"))] # [inline] pub fn clock_gettime_dynamic (id : DynamicClockId < '_ >) -> io :: Result < Timespec > { backend :: time :: syscalls :: clock_gettime_dynamic (id) }
    };
}

clock_gettime_dynamic!()