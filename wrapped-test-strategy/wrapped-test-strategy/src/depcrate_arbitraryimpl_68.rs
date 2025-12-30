// Generated macro for impl_68 (impl)
macro_rules! Depcrate_arbitraryimpl_68 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_68"}
// Dependencies: {}
impl ToTokens for StrategyExpr { fn to_tokens (& self , tokens : & mut TokenStream) { let expr = & self . expr ; let expr = if self . is_jast { let ty = & self . ty ; quote ! (proptest :: strategy :: Just ::<# ty > (# expr)) } else { quote ! (# expr) } ; let expr = if self . filters . is_empty () { expr } else { let mut filter_lets = Vec :: new () ; let var : Ident = parse_quote ! { _s } ; for filter in & self . filters { filter_lets . push (filter . make_let_fn (& var)) ; } quote ! { { let # var = # expr ; # (# filter_lets) * # var } } } ; tokens . extend (expr) ; } }
};
}
