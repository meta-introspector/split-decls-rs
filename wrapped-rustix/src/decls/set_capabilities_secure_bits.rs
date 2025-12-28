macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_capabilities_secure_bits {
    () => {
        deps!();
        # [doc = " Set the `securebits` flags of the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_SECUREBITS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_SECUREBITS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn set_capabilities_secure_bits (bits : CapabilitiesSecureBits) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_SECUREBITS , bits . bits () as usize as * mut _) } . map (| _r | ()) }
    };
}

set_capabilities_secure_bits!()