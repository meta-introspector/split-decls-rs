macro_rules! EXIT_SUCCESS {
    () => {
        # [doc = " `EXIT_SUCCESS` for use with [`exit_group`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/stdlib.h.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html"] pub const EXIT_SUCCESS : i32 = backend :: c :: EXIT_SUCCESS ;
    };
}

EXIT_SUCCESS!()