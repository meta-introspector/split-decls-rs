// Generated macro for impl_275 (impl)
macro_rules! Depcrate_syn2macroimpl_275 {
() => {
// Module: crate::syn2macro
// Provides: {"impl_275"}
// Dependencies: {}
impl SecureExecution for StrictSecurity { fn check_permission (& self , operation : & AstOperation) -> bool { self . allowed_operations . iter () . any (| op | { std :: mem :: discriminant (op) == std :: mem :: discriminant (operation) }) } fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T { Ok (f ()) } }
};
}
