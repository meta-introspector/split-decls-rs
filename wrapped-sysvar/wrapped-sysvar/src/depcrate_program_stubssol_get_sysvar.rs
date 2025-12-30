// Generated macro for sol_get_sysvar (function)
macro_rules! Depcrate_program_stubssol_get_sysvar {
() => {
// Module: crate::program_stubs
// Provides: {"sol_get_sysvar"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) fn sol_get_sysvar (sysvar_id_addr : * const u8 , var_addr : * mut u8 , offset : u64 , length : u64 ,) -> u64 { SYSCALL_STUBS . read () . unwrap () . sol_get_sysvar (sysvar_id_addr , var_addr , offset , length) }
};
}
