// Generated macro for check_type (macro)
macro_rules! Depcrate_check_typescheck_type {
() => {
// Module: crate::check_types
// Provides: {"check_type"}
// Dependencies: {}
# [doc = " Check that the size and alignment of a type match the `sys` bindings."] macro_rules ! check_type { ($ struct : ident) => { assert_eq_size ! ($ struct , c ::$ struct) ; assert_eq_align ! ($ struct , c ::$ struct) ; } ; }
};
}
