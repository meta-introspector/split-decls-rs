macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! nice {
    () => {
        deps!();
        # [doc = " `nice(inc)`—Adjust the scheduling priority of the current process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/nice.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/nice.2.html"] # [inline] pub fn nice (inc : i32) -> io :: Result < i32 > { backend :: process :: syscalls :: nice (inc) }
    };
}

nice!()