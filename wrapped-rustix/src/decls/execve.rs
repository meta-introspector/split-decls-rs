macro_rules! execve {
    () => {
        # [doc = " `execve(path.as_c_str(), argv, envp)`—Execute a new command using the"] # [doc = " current process."] # [doc = ""] # [doc = " Taking raw-pointers-to-raw-pointers is convenient for c-scape, but we"] # [doc = " should think about potentially a more Rust-idiomatic API if this is ever"] # [doc = " made public."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `argv` and `envp` pointers must point to NUL-terminated arrays, and"] # [doc = " their contents must be pointers to NUL-terminated byte arrays."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/execve.2.html"] # [inline] # [must_use] pub unsafe fn execve (path : & CStr , argv : * const * const u8 , envp : * const * const u8) -> io :: Errno { backend :: runtime :: syscalls :: execve (path , argv , envp) }
    };
}

execve!()