macro_rules! exit_group {
    () => {
        # [doc = " Exit all the threads in the current process' thread group."] # [doc = ""] # [doc = " This is equivalent to `_exit` and `_Exit` in libc."] # [doc = ""] # [doc = " This does not call any `__cxa_atexit`, `atexit`, or any other destructors."] # [doc = " Most programs should use [`std::process::exit`] instead of calling this"] # [doc = " directly."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX `_Exit`]"] # [doc = "  - [Linux `exit_group`]"] # [doc = "  - [Linux `_Exit`]"] # [doc = ""] # [doc = " [POSIX `_Exit`]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/_Exit.html"] # [doc = " [Linux `exit_group`]: https://man7.org/linux/man-pages/man2/exit_group.2.html"] # [doc = " [Linux `_Exit`]: https://man7.org/linux/man-pages/man2/_Exit.2.html"] # [doc (alias = "_exit" , alias = "_Exit")] # [inline] pub fn exit_group (status : i32) -> ! { backend :: runtime :: syscalls :: exit_group (status) }
    };
}

exit_group!()