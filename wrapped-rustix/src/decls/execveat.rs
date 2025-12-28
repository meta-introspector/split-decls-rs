macro_rules! execveat {
    () => {
        # [doc = " `execveat(dirfd, path.as_c_str(), argv, envp, flags)`—Execute a new"] # [doc = " command using the current process."] # [doc = ""] # [doc = " Taking raw-pointers-to-raw-pointers is convenient for c-scape, but we"] # [doc = " should think about potentially a more Rust-idiomatic API if this is ever"] # [doc = " made public."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `argv` and `envp` pointers must point to NUL-terminated arrays, and"] # [doc = " their contents must be pointers to NUL-terminated byte arrays."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/execveat.2.html"] # [inline] # [cfg (feature = "fs")] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] # [must_use] pub unsafe fn execveat < Fd : AsFd > (dirfd : Fd , path : & CStr , argv : * const * const u8 , envp : * const * const u8 , flags : AtFlags ,) -> io :: Errno { backend :: runtime :: syscalls :: execveat (dirfd . as_fd () , path , argv , envp , flags) }
    };
}

execveat!();