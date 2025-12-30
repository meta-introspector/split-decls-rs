// Generated macro for impl_52 (impl)
macro_rules! Depcrate_arbitraryimpl_52 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_52"}
// Dependencies: {}
impl AnyArgs { fn empty () -> Self { Self { initializer : None , setters : HashMap :: new () , } } fn into_strategy (self , ty : & StrategyValueType) -> TokenStream { if self . initializer . is_none () && self . setters . is_empty () { ty . any () } else { let init = self . initializer . unwrap_or_else (| | parse_quote ! (std :: default :: Default :: default ())) . to_token_stream () ; if self . setters . is_empty () { ty . any_with (init) } else { let mut setters : Vec < _ > = self . setters . into_iter () . collect () ; setters . sort_by (| v0 , v1 | v0 . 0 . cmp (& v1 . 0)) ; let setters = setters . into_iter () . map (| (name , expr) | { let member = Member :: Named (to_valid_ident (& name) . unwrap ()) ; let expr = & expr . value ; quote ! (_any_args .# member = # expr ;) }) ; let any_with = ty . any_with (quote ! (_any_args)) ; let any_with_args_let = ty . any_with_args_let (quote ! (_any_args) , init) ; quote ! { { # any_with_args_let ; # (# setters) * # any_with } } } } } }
};
}
