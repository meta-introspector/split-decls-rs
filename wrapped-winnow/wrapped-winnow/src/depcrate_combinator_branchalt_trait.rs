// Generated macro for alt_trait (macro)
macro_rules! Depcrate_combinator_branchalt_trait {
() => {
// Module: crate::combinator::branch
// Provides: {"alt_trait"}
// Dependencies: {}
macro_rules ! alt_trait (($ first : ident $ second : ident $ ($ id : ident) +) => (alt_trait ! (__impl $ first $ second ; $ ($ id) +) ;) ; (__impl $ ($ current : ident) *; $ head : ident $ ($ id : ident) +) => (alt_trait_impl ! ($ ($ current) *) ; alt_trait ! (__impl $ ($ current) * $ head ; $ ($ id) +) ;) ; (__impl $ ($ current : ident) *; $ head : ident) => (alt_trait_impl ! ($ ($ current) *) ; alt_trait_impl ! ($ ($ current) * $ head) ;) ;) ;
};
}
