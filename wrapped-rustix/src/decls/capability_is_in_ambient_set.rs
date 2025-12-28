macro_rules! deps {
    () => {
        Result!();
        CompatCapability!();
    };
}

macro_rules! capability_is_in_ambient_set {
    () => {
        deps!();
        # [doc = " Check if the specified capability is in the ambient set."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_CAP_AMBIENT,PR_CAP_AMBIENT_IS_SET,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_CAP_AMBIENT,PR_CAP_AMBIENT_IS_SET,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn capability_is_in_ambient_set (capability : impl CompatCapability) -> io :: Result < bool > { let capset = capability . as_capability_set (private :: Token) . bits () ; if capset . count_ones () != 1 { return Err (Errno :: INVAL) ; } let cap = capset . trailing_zeros () ; unsafe { prctl_3args (PR_CAP_AMBIENT , PR_CAP_AMBIENT_IS_SET as * mut _ , cap as usize as * mut _ ,) } . map (| r | r != 0) }
    };
}

capability_is_in_ambient_set!()