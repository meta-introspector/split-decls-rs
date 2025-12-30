// Generated macro for impl_29 (impl)
macro_rules! Depcrate_attrimpl_29 {
() => {
// Module: crate::attr
// Provides: {"impl_29"}
// Dependencies: {}
impl Parse for Skips { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let _ = input . parse :: < kw :: skip > () ; let content ; let _ = syn :: parenthesized ! (content in input) ; let names = content . parse_terminated (Ident :: parse_any , Token ! [,]) ? ; let mut skips = HashSet :: new () ; for name in names { if skips . contains (& name) { return Err (syn :: Error :: new (name . span () , "tried to skip the same field twice" ,)) ; } else { skips . insert (name) ; } } Ok (Self (skips)) } }
};
}
