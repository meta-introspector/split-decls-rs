// Generated macro for get_sysvar (function)
macro_rules! Depcrateget_sysvar {
() => {
// Module: crate
// Provides: {"get_sysvar"}
// Dependencies: {}
# [doc = " Handler for retrieving a slice of sysvar data from the `sol_get_sysvar`"] # [doc = " syscall."] pub fn get_sysvar (dst : & mut [u8] , sysvar_id : & Pubkey , offset : u64 , length : u64 ,) -> Result < () , solana_program_error :: ProgramError > { if dst . len () < length as usize { return Err (solana_program_error :: ProgramError :: InvalidArgument) ; } let sysvar_id = sysvar_id as * const _ as * const u8 ; let var_addr = dst as * mut _ as * mut u8 ; # [cfg (target_os = "solana")] let result = unsafe { solana_define_syscall :: definitions :: sol_get_sysvar (sysvar_id , var_addr , offset , length) } ; # [cfg (not (target_os = "solana"))] let result = crate :: program_stubs :: sol_get_sysvar (sysvar_id , var_addr , offset , length) ; match result { solana_program_entrypoint :: SUCCESS => Ok (()) , OFFSET_LENGTH_EXCEEDS_SYSVAR => Err (solana_program_error :: ProgramError :: InvalidArgument) , SYSVAR_NOT_FOUND => Err (solana_program_error :: ProgramError :: UnsupportedSysvar) , _ => Err (solana_program_error :: ProgramError :: UnsupportedSysvar) , } }
};
}
