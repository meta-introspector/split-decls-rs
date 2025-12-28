macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! unaligned_access_control {
    () => {
        deps!();
        # [doc = " Get unaligned access control bits."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_UNALIGN,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_UNALIGN,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_UNALIGN")] pub fn unaligned_access_control () -> io :: Result < UnalignedAccessControl > { let r = unsafe { prctl_get_at_arg2_optional :: < c_uint > (PR_GET_UNALIGN) ? } ; UnalignedAccessControl :: from_bits (r) . ok_or (io :: Errno :: RANGE) }
    };
}

unaligned_access_control!();