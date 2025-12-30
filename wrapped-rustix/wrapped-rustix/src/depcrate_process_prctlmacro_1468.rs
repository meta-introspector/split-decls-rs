// Generated macro for macro_1468 (macro)
macro_rules! Depcrate_process_prctlmacro_1468 {
() => {
// Module: crate::process::prctl
// Provides: {"macro_1468"}
// Dependencies: {}
bitflags ! { # [doc = " Zero means the processors are not vulnerable."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct SpeculationFeatureState : u32 { # [doc = " Mitigation can be controlled per thread by"] # [doc = " [`control_speculative_feature`]."] const PRCTL = 1_u32 << 0 ; # [doc = " The speculation feature is enabled, mitigation is disabled."] const ENABLE = 1_u32 << 1 ; # [doc = " The speculation feature is disabled, mitigation is enabled."] const DISABLE = 1_u32 << 2 ; # [doc = " The speculation feature is disabled, mitigation is enabled, and it"] # [doc = " cannot be undone."] const FORCE_DISABLE = 1_u32 << 3 ; # [doc = " The speculation feature is disabled, mitigation is enabled, and the"] # [doc = " state will be cleared on `execve`."] const DISABLE_NOEXEC = 1_u32 << 4 ; } }
};
}
