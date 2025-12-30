// Generated macro for impl_272 (impl)
macro_rules! Depcrate_syn2macroimpl_272 {
() => {
// Module: crate::syn2macro
// Provides: {"impl_272"}
// Dependencies: {}
impl SecureExecution for DefaultSecurity { fn check_permission (& self , _operation : & AstOperation) -> bool { true } fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T { Ok (f ()) } }
};
}
