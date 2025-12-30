// Generated macro for expect_opt (macro)
macro_rules! Depcrate_internal_macrosexpect_opt {
() => {
// Module: crate::internal_macros
// Provides: {"expect_opt"}
// Dependencies: {}
# [doc = " Try to unwrap an expression, panicking if not possible."] # [doc = ""] # [doc = " This is similar to `$e.expect($message)`, but is usable in `const` contexts."] macro_rules ! expect_opt { ($ e : expr , $ message : literal) => { match $ e { Some (value) => value , None => crate :: expect_failed ($ message) , } } ; }
};
}
