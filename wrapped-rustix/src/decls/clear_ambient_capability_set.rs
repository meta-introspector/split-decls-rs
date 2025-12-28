macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! clear_ambient_capability_set {
    () => {
        deps!();
        # [doc = " Remove all capabilities from the ambient set."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_CAP_AMBIENT,PR_CAP_AMBIENT_CLEAR_ALL,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_CAP_AMBIENT,PR_CAP_AMBIENT_CLEAR_ALL,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn clear_ambient_capability_set () -> io :: Result < () > { unsafe { prctl_2args (PR_CAP_AMBIENT , PR_CAP_AMBIENT_CLEAR_ALL as * mut _) } . map (| _r | ()) }
    };
}

clear_ambient_capability_set!();