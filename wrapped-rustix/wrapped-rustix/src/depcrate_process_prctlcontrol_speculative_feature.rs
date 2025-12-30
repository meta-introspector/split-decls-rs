// Generated macro for control_speculative_feature (function)
macro_rules! Depcrate_process_prctlcontrol_speculative_feature {
() => {
// Module: crate::process::prctl
// Provides: {"control_speculative_feature"}
// Dependencies: {}
# [doc = " Sets the state of the speculation misfeature."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_SPECULATION_CTRL,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_SPECULATION_CTRL,…)`]: https://www.kernel.org/doc/html/v6.13/userspace-api/spec_ctrl.html"] # [inline] # [doc (alias = "PR_SET_SPECULATION_CTRL")] pub fn control_speculative_feature (feature : SpeculationFeature , config : SpeculationFeatureControl ,) -> io :: Result < () > { let feature = feature as usize as * mut _ ; let config = config . bits () as usize as * mut _ ; unsafe { prctl_3args (PR_SET_SPECULATION_CTRL , feature , config) } . map (| _r | ()) }
};
}
