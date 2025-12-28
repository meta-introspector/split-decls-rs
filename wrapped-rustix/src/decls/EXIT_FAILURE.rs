macro_rules! EXIT_FAILURE {
    () => {
        # [doc = " `EXIT_FAILURE` for use with [`exit_group`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/stdlib.h.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html"] pub const EXIT_FAILURE : i32 = backend :: c :: EXIT_FAILURE ;
    };
}

EXIT_FAILURE!()