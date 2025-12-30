// Generated macro for impl_34 (impl)
macro_rules! Depcrate_helpers_metadataimpl_34 {
() => {
// Module: crate::helpers::metadata
// Provides: {"impl_34"}
// Dependencies: {}
impl Parse for EnumDiscriminantsMeta { fn parse (input : ParseStream) -> syn :: Result < Self > { if input . peek (kw :: derive) { let _kw = input . parse () ? ; let content ; parenthesized ! (content in input) ; let paths = content . parse_terminated (Path :: parse , Token ! [,]) ? ; Ok (EnumDiscriminantsMeta :: Derive { _kw , paths : paths . into_iter () . collect () , }) } else if input . peek (kw :: name) { let kw = input . parse () ? ; let content ; parenthesized ! (content in input) ; let name = content . parse () ? ; Ok (EnumDiscriminantsMeta :: Name { kw , name }) } else if input . peek (kw :: vis) { let kw = input . parse () ? ; let content ; parenthesized ! (content in input) ; let vis = content . parse () ? ; Ok (EnumDiscriminantsMeta :: Vis { kw , vis }) } else if input . peek (kw :: doc) { let _kw = input . parse () ? ; input . parse :: < Token ! [=] > () ? ; let doc = input . parse () ? ; Ok (EnumDiscriminantsMeta :: Doc { _kw , doc }) } else { let path = input . parse () ? ; let content ; parenthesized ! (content in input) ; let nested = content . parse () ? ; Ok (EnumDiscriminantsMeta :: Other { path , nested }) } } }
};
}
