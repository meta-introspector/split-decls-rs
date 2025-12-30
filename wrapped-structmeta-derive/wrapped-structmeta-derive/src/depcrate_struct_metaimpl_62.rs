// Generated macro for impl_62 (impl)
macro_rules! Depcrate_struct_metaimpl_62 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_62"}
// Dependencies: {}
impl Parse for ArgForStruct { fn parse (input : ParseStream) -> Result < Self > { if input . peek (kw :: dump) { return Ok (Self :: Dump (input . parse () ?)) ; } if input . peek (kw :: name_filter) { let kw_name_filter : kw :: name_filter = input . parse () ? ; let _eq : Token ! [=] = input . parse () ? ; let s : LitStr = input . parse () ? ; let value = match s . value () . as_str () { "snake_case" => NameFilter :: SnakeCase , _ => { bail ! (s . span () , "expected \"snake_case\"") } } ; return Ok (Self :: NameFilter { span : kw_name_filter . span , value , }) ; } Err (input . error ("usage : #[struct_meta(dump)]")) } }
};
}
