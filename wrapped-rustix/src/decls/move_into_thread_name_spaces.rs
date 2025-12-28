macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! move_into_thread_name_spaces {
    () => {
        deps!();
        # [doc = " Atomically move the calling thread into one or more of the same namespaces"] # [doc = " as the thread referred to by `fd`."] # [doc = ""] # [doc = " `fd` must refer to a thread ID. See: `pidfd_open` and `clone`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setns.2.html"] # [doc (alias = "setns")] pub fn move_into_thread_name_spaces (fd : BorrowedFd < '_ > , allowed_types : ThreadNameSpaceType ,) -> io :: Result < () > { syscalls :: setns (fd , allowed_types . bits () as c_int) . map (| _r | ()) }
    };
}

move_into_thread_name_spaces!();