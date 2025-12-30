// Generated macro for set_floating_point_emulation_control (function)
macro_rules! Depcrate_process_prctlset_floating_point_emulation_control {
() => {
// Module: crate::process::prctl
// Provides: {"set_floating_point_emulation_control"}
// Dependencies: {}
# [doc = " Set floating point emulation control bits."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_FPEMU,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_FPEMU,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_FPEMU")] pub fn set_floating_point_emulation_control (config : FloatingPointEmulationControl ,) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_FPEMU , config . bits () as usize as * mut _) } . map (| _r | ()) }
};
}
