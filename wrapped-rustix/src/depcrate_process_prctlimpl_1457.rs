// Generated macro for impl_1457 (impl)
macro_rules! Depcrate_process_prctlimpl_1457 {
() => {
// Module: crate::process::prctl
// Provides: {"impl_1457"}
// Dependencies: {}
impl TryFrom < u32 > for FloatingPointMode { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_FP_MODE_FR => Ok (Self :: FloatingPointRegisters) , PR_FP_MODE_FRE => Ok (Self :: FloatingPointEmulation) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
