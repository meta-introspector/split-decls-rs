// Generated macro for floating_point_emulation_control (function)
macro_rules! Depcrate_process_prctlfloating_point_emulation_control {
() => {
// Module: crate::process::prctl
// Provides: {"floating_point_emulation_control"}
// Dependencies: {}
# [doc = " Get floating point emulation control bits."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_FPEMU,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_FPEMU,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_FPEMU")] pub fn floating_point_emulation_control () -> io :: Result < FloatingPointEmulationControl > { let r = unsafe { prctl_get_at_arg2_optional :: < c_uint > (PR_GET_FPEMU) ? } ; FloatingPointEmulationControl :: from_bits (r) . ok_or (io :: Errno :: RANGE) }
};
}
