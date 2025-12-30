// Generated macro for impl_26 (impl)
macro_rules! Depcrate_astimpl_26 {
() => {
// Module: crate::ast
// Provides: {"impl_26"}
// Dependencies: {}
impl Variant < '_ > { pub fn name (& self) -> Name < '_ > { Name (self . serde_attrs . name ()) } pub fn is_unit (& self) -> bool { matches ! (self . style , serde_ast :: Style :: Unit) } pub fn add_mutators (& self , mutators : & mut Vec < TokenStream >) { self . attrs . common . add_mutators (mutators) ; } pub fn with_contract_check (& self , action : TokenStream) -> TokenStream { with_contract_check (self . serde_attrs . skip_deserializing () , self . serde_attrs . skip_serializing () , action ,) } }
};
}
