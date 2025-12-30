// Generated macro for impl_66 (impl)
macro_rules! Depcrate_struct_metaimpl_66 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_66"}
// Dependencies: {}
impl Parse for ArgForField { fn parse (input : ParseStream) -> Result < Self > { if input . peek (kw :: name) && input . peek2 (Token ! [=]) { let name_token = input . parse () ? ; let eq_token = input . parse () ? ; let value = input . parse () ? ; Ok (Self :: Name { _name_token : name_token , _eq_token : eq_token , value , }) } else if input . peek (kw :: unnamed) { Ok (Self :: Unnamed { _unnamed_token : input . parse () ? , }) } else { Err (input . error ("expected `name = \"...\"` or `unnamed`.")) } } }
};
}
