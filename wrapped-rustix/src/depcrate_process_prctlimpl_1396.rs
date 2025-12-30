// Generated macro for impl_1396 (impl)
macro_rules! Depcrate_process_prctlimpl_1396 {
() => {
// Module: crate::process::prctl
// Provides: {"impl_1396"}
// Dependencies: {}
impl TryFrom < u32 > for EndianMode { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_ENDIAN_BIG => Ok (Self :: Big) , PR_ENDIAN_LITTLE => Ok (Self :: Little) , PR_ENDIAN_PPC_LITTLE => Ok (Self :: PowerPCLittle) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
