macro_rules! deps {
    () => {
        Result!();
        CompatCapability!();
    };
}

macro_rules! capability_is_in_bounding_set {
    () => {
        deps!();
        # [doc = " Check if the specified capability is in the calling thread's capability"] # [doc = " bounding set."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_CAPBSET_READ,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_CAPBSET_READ,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn capability_is_in_bounding_set (capability : impl CompatCapability) -> io :: Result < bool > { let capset = capability . as_capability_set (private :: Token) . bits () ; if capset . count_ones () != 1 { return Err (Errno :: INVAL) ; } let cap = capset . trailing_zeros () ; unsafe { prctl_2args (PR_CAPBSET_READ , cap as usize as * mut _) } . map (| r | r != 0) }
    };
}

capability_is_in_bounding_set!();