macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! get_keep_capabilities {
    () => {
        deps!();
        # [doc = " Get the current state of the calling thread's `keep capabilities` flag."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_KEEPCAPS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_KEEPCAPS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn get_keep_capabilities () -> io :: Result < bool > { unsafe { prctl_1arg (PR_GET_KEEPCAPS) } . map (| r | r != 0) }
    };
}

get_keep_capabilities!();