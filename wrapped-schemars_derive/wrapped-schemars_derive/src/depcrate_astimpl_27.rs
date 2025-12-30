// Generated macro for impl_27 (impl)
macro_rules! Depcrate_astimpl_27 {
() => {
// Module: crate::ast
// Provides: {"impl_27"}
// Dependencies: {}
impl Field < '_ > { pub fn name (& self) -> Name < '_ > { Name (self . serde_attrs . name ()) } pub fn add_mutators (& self , mutators : & mut Vec < TokenStream >) { self . attrs . common . add_mutators (mutators) ; self . attrs . validation . add_mutators (mutators) ; if self . serde_attrs . skip_deserializing () { mutators . push (quote ! { # SCHEMA . insert ("readOnly" . into () , true . into ()) ; }) ; } if self . serde_attrs . skip_serializing () { mutators . push (quote ! { # SCHEMA . insert ("writeOnly" . into () , true . into ()) ; }) ; } } pub fn with_contract_check (& self , action : TokenStream) -> TokenStream { with_contract_check (self . serde_attrs . skip_deserializing () , self . serde_attrs . skip_serializing () , action ,) } }
};
}
