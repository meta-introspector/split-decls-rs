macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! unshare_unsafe {
    () => {
        deps!();
        # [doc = " `unshare(flags)`—Disassociate parts of the current thread's execution"] # [doc = " context with other threads."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " When using `UnshareFlags::FILES`, this function can cause one thread to be"] # [doc = " unable to use file descriptors created on a different thread. Callers must"] # [doc = " ensure that threads never observe file descriptors from unshared tables."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/unshare.2.html"] pub unsafe fn unshare_unsafe (flags : UnshareFlags) -> io :: Result < () > { syscalls :: unshare (flags) }
    };
}

unshare_unsafe!()