// Generated macro for impl_111 (impl)
macro_rules! Depcrate_androidimpl_111 {
() => {
// Module: crate::android
// Provides: {"impl_111"}
// Dependencies: {}
impl Global { fn env (& self) -> Result < AttachGuard < '_ > , Error > { let vm = match self { Global :: Internal { java_vm , .. } => java_vm , Global :: External (global) => global . java_vm () , } ; Ok (vm . attach_current_thread () ?) } fn context (& self) -> Result < (GlobalContext , AttachGuard < '_ >) , Error > { let env = self . env () ? ; let context = match self { Global :: Internal { context , .. } => context , Global :: External (global) => global . context () , } ; let loader = match self { Global :: Internal { loader , .. } => loader , Global :: External (global) => global . class_loader () , } ; Ok ((GlobalContext { context : env . new_global_ref (context) ? , loader : env . new_global_ref (loader) ? , } , env ,)) } }
};
}
