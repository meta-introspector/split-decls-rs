macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! is_io_flusher {
    () => {
        deps!();
        # [doc = " Get the `IO_FLUSHER` state of the caller."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_IO_FLUSHER,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_IO_FLUSHER,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_IO_FLUSHER")] pub fn is_io_flusher () -> io :: Result < bool > { unsafe { prctl_1arg (PR_GET_IO_FLUSHER) } . map (| r | r != 0) }
    };
}

is_io_flusher!()