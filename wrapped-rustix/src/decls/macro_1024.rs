macro_rules! macro_1024 {
    () => {
        bitflags ! { # [doc = " `PR_SPEC_*` flags for use with [`control_speculative_feature`]."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct SpeculationFeatureControl : u32 { # [doc = " The speculation feature is enabled, mitigation is disabled."] const ENABLE = 1_u32 << 1 ; # [doc = " The speculation feature is disabled, mitigation is enabled."] const DISABLE = 1_u32 << 2 ; # [doc = " The speculation feature is disabled, mitigation is enabled, and it"] # [doc = " cannot be undone."] const FORCE_DISABLE = 1_u32 << 3 ; # [doc = " The speculation feature is disabled, mitigation is enabled, and the"] # [doc = " state will be cleared on `execve`."] const DISABLE_NOEXEC = 1_u32 << 4 ; } }
    };
}

macro_1024!();