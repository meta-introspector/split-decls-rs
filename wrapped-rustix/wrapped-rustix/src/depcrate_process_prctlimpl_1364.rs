// Generated macro for impl_1364 (impl)
macro_rules! Depcrate_process_prctlimpl_1364 {
() => {
// Module: crate::process::prctl
// Provides: {"impl_1364"}
// Dependencies: {}
impl TryFrom < i32 > for DumpableBehavior { type Error = io :: Errno ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { SUID_DUMP_DISABLE => Ok (Self :: NotDumpable) , SUID_DUMP_USER => Ok (Self :: Dumpable) , SUID_DUMP_ROOT => Ok (Self :: DumpableReadableOnlyByRoot) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
