// Generated macro for sol_invoke_signed (function)
macro_rules! Depcrate_program_stubssol_invoke_signed {
() => {
// Module: crate::program_stubs
// Provides: {"sol_invoke_signed"}
// Dependencies: {}
pub fn sol_invoke_signed (instruction : & Instruction , account_infos : & [AccountInfo] , signers_seeds : & [& [& [u8]]] ,) -> ProgramResult { SYSCALL_STUBS . read () . unwrap () . sol_invoke_signed (instruction , account_infos , signers_seeds) }
};
}
