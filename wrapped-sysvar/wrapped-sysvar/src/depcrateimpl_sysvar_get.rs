// Generated macro for impl_sysvar_get (macro)
macro_rules! Depcrateimpl_sysvar_get {
() => {
// Module: crate
// Provides: {"impl_sysvar_get"}
// Dependencies: {}
# [doc = " Implements the [`Sysvar::get`] method for both SBF and host targets."] # [macro_export] macro_rules ! impl_sysvar_get { ($ syscall_name : ident) => { fn get () -> Result < Self , $ crate :: __private :: ProgramError > { let mut var = Self :: default () ; let var_addr = & mut var as * mut _ as * mut u8 ; # [cfg (target_os = "solana")] let result = unsafe { $ crate :: __private :: definitions ::$ syscall_name (var_addr) } ; # [cfg (not (target_os = "solana"))] let result = $ crate :: program_stubs ::$ syscall_name (var_addr) ; match result { $ crate :: __private :: SUCCESS => Ok (var) , _ => Err ($ crate :: __private :: ProgramError :: UnsupportedSysvar) , } } } ; }
};
}
