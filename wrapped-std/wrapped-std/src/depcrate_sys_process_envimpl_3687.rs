// Generated macro for impl_3687 (impl)
macro_rules! Depcrate_sys_process_envimpl_3687 {
() => {
// Module: crate::sys::process::env
// Provides: {"impl_3687"}
// Dependencies: {}
impl fmt :: Debug for CommandEnv { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut debug_command_env = f . debug_struct ("CommandEnv") ; debug_command_env . field ("clear" , & self . clear) . field ("vars" , & self . vars) ; debug_command_env . finish () } }
};
}
