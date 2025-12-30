// Generated macro for const_try (macro)
macro_rules! Depcrate_internal_macrosconst_try {
() => {
// Module: crate::internal_macros
// Provides: {"const_try"}
// Dependencies: {}
# [doc = " Try to unwrap an expression, returning if not possible."] # [doc = ""] # [doc = " This is similar to the `?` operator, but does not perform `.into()`. Because of this, it is"] # [doc = " usable in `const` contexts."] macro_rules ! const_try { ($ e : expr) => { match $ e { Ok (value) => value , Err (error) => return Err (error) , } } ; }
};
}
