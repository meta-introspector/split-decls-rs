// Generated macro for impl_34 (impl)
macro_rules! Depcrate_attrimpl_34 {
() => {
// Module: crate::attr
// Provides: {"impl_34"}
// Dependencies: {}
impl Parse for Fields { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let _ = input . parse :: < kw :: fields > () ; let content ; let _ = syn :: parenthesized ! (content in input) ; let fields = content . parse_terminated (Field :: parse , Token ! [,]) ? ; Ok (Self (fields)) } }
};
}
