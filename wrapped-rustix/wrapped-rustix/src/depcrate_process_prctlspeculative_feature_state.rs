// Generated macro for speculative_feature_state (function)
macro_rules! Depcrate_process_prctlspeculative_feature_state {
() => {
// Module: crate::process::prctl
// Provides: {"speculative_feature_state"}
// Dependencies: {}
# [doc = " Get the state of the speculation misfeature."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_SPECULATION_CTRL,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_SPECULATION_CTRL,…)`]: https://www.kernel.org/doc/html/v6.13/userspace-api/spec_ctrl.html"] # [inline] # [doc (alias = "PR_GET_SPECULATION_CTRL")] pub fn speculative_feature_state (feature : SpeculationFeature ,) -> io :: Result < Option < SpeculationFeatureState > > { let r = unsafe { prctl_2args (PR_GET_SPECULATION_CTRL , feature as usize as * mut _) ? } as c_uint ; Ok (SpeculationFeatureState :: from_bits (r)) }
};
}
