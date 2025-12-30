// Generated macro for const_try_opt (macro)
macro_rules! Depcrate_internal_macrosconst_try_opt {
() => {
// Module: crate::internal_macros
// Provides: {"const_try_opt"}
// Dependencies: {}
# [doc = " Try to unwrap an expression, returning if not possible."] # [doc = ""] # [doc = " This is similar to the `?` operator, but is usable in `const` contexts."] macro_rules ! const_try_opt { ($ e : expr) => { match $ e { Some (value) => value , None => return None , } } ; }
};
}
