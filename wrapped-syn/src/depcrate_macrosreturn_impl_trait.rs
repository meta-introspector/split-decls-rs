// Generated macro for return_impl_trait (macro)
macro_rules! Depcrate_macrosreturn_impl_trait {
() => {
// Module: crate::macros
// Provides: {"return_impl_trait"}
// Dependencies: {}
# [cfg (any (feature = "full" , feature = "derive"))] macro_rules ! return_impl_trait { ($ (# [$ attr : meta]) * $ vis : vis fn $ name : ident $ args : tt -> $ impl_trait : ty [$ concrete : ty] $ body : block) => { # [cfg (not (docsrs))] $ (# [$ attr]) * $ vis fn $ name $ args -> $ concrete $ body # [cfg (docsrs)] $ (# [$ attr]) * $ vis fn $ name $ args -> $ impl_trait $ body } ; }
};
}
