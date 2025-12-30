// Generated macro for impl_268 (impl)
macro_rules! Depcrate_syn2macroimpl_268 {
() => {
// Module: crate::syn2macro
// Provides: {"impl_268"}
// Dependencies: {}
impl SecurityContext { pub fn check_permission (& self , operation : & AstOperation) -> bool { match self { SecurityContext :: Default => true , SecurityContext :: Strict (allowed) => { allowed . iter () . any (| op | { std :: mem :: discriminant (op) == std :: mem :: discriminant (operation) }) } } } }
};
}
