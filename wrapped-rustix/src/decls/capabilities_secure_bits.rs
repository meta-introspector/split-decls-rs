macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! capabilities_secure_bits {
    () => {
        deps!();
        # [doc = " Get the `securebits` flags of the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_SECUREBITS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_SECUREBITS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn capabilities_secure_bits () -> io :: Result < CapabilitiesSecureBits > { let r = unsafe { prctl_1arg (PR_GET_SECUREBITS) ? } as c_uint ; CapabilitiesSecureBits :: from_bits (r) . ok_or (io :: Errno :: RANGE) }
    };
}

capabilities_secure_bits!();