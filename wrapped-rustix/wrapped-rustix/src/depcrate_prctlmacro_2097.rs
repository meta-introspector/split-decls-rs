// Generated macro for macro_2097 (macro)
macro_rules! Depcrate_prctlmacro_2097 {
() => {
// Module: crate::prctl
// Provides: {"macro_2097"}
// Dependencies: {}
# [cfg (linux_raw_dep)] bitflags ! { # [doc = " `PR_PAC_AP*`"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct PointerAuthenticationKeys : u32 { # [doc = " `PR_PAC_APIAKEY`—Instruction authentication key `A`."] const INSTRUCTION_AUTHENTICATION_KEY_A = linux_raw_sys :: prctl :: PR_PAC_APIAKEY ; # [doc = " `PR_PAC_APIBKEY`—Instruction authentication key `B`."] const INSTRUCTION_AUTHENTICATION_KEY_B = linux_raw_sys :: prctl :: PR_PAC_APIBKEY ; # [doc = " `PR_PAC_APDAKEY`—Data authentication key `A`."] const DATA_AUTHENTICATION_KEY_A = linux_raw_sys :: prctl :: PR_PAC_APDAKEY ; # [doc = " `PR_PAC_APDBKEY`—Data authentication key `B`."] const DATA_AUTHENTICATION_KEY_B = linux_raw_sys :: prctl :: PR_PAC_APDBKEY ; # [doc = " `PR_PAC_APGAKEY`—Generic authentication `A` key."] const GENERIC_AUTHENTICATION_KEY_A = linux_raw_sys :: prctl :: PR_PAC_APGAKEY ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
