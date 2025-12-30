// Generated macro for endian_mode (function)
macro_rules! Depcrate_process_prctlendian_mode {
() => {
// Module: crate::process::prctl
// Provides: {"endian_mode"}
// Dependencies: {}
# [doc = " Get the endianness of the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_ENDIAN,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_ENDIAN,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_ENDIAN")] pub fn endian_mode () -> io :: Result < EndianMode > { unsafe { prctl_get_at_arg2 :: < c_uint , _ > (PR_GET_ENDIAN) } }
};
}
