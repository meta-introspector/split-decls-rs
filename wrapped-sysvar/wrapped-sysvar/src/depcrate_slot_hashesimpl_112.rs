// Generated macro for impl_112 (impl)
macro_rules! Depcrate_slot_hashesimpl_112 {
() => {
// Module: crate::slot_hashes
// Provides: {"impl_112"}
// Dependencies: {}
# [cfg (feature = "bincode")] impl SysvarSerialize for SlotHashes { fn size_of () -> usize { SYSVAR_LEN } fn from_account_info (_account_info : & AccountInfo ,) -> Result < Self , solana_program_error :: ProgramError > { Err (solana_program_error :: ProgramError :: UnsupportedSysvar) } }
};
}
