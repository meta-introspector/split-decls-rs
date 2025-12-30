// Generated macro for ast_enum (macro)
macro_rules! Depcrate_macrosast_enum {
() => {
// Module: crate::macros
// Provides: {"ast_enum"}
// Dependencies: {}
# [cfg (any (feature = "full" , feature = "derive"))] macro_rules ! ast_enum { ($ (# [$ enum_attr : meta]) * $ pub : ident $ enum : ident $ name : ident $ body : tt) => { check_keyword_matches ! (pub $ pub) ; check_keyword_matches ! (enum $ enum) ; $ (# [$ enum_attr]) * $ pub $ enum $ name $ body } ; }
};
}
