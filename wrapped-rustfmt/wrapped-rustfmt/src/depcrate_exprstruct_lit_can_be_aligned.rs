// Generated macro for struct_lit_can_be_aligned (function)
macro_rules! Depcrate_exprstruct_lit_can_be_aligned {
() => {
// Module: crate::expr
// Provides: {"struct_lit_can_be_aligned"}
// Dependencies: {}
fn struct_lit_can_be_aligned (fields : & [ast :: ExprField] , has_base : bool) -> bool { ! has_base && fields . iter () . all (| field | ! field . is_shorthand) }
};
}
