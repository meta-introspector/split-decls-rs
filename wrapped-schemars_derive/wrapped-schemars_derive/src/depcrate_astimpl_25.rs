// Generated macro for impl_25 (impl)
macro_rules! Depcrate_astimpl_25 {
() => {
// Module: crate::ast
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > Container < 'a > { pub fn from_ast (item : & 'a syn :: DeriveInput) -> syn :: Result < Container < 'a > > { let ctxt = Ctxt :: new () ; let result = serde_ast :: Container :: from_ast (& ctxt , item , Derive :: Deserialize) . ok_or (()) . map (| serde | Self :: from_serde (& ctxt , serde)) ; ctxt . check () . map (| () | result . expect ("from_ast set no errors on Ctxt, so should have returned Ok")) } pub fn transparent_field (& 'a self) -> Option < & 'a Field < 'a > > { if self . serde_attrs . transparent () { if let Data :: Struct (_ , fields) = & self . data { return fields . iter () . find (| f | f . serde_attrs . transparent ()) ; } } None } pub fn add_mutators (& self , mutators : & mut Vec < TokenStream >) { self . attrs . common . add_mutators (mutators) ; } pub fn name (& 'a self) -> std :: borrow :: Cow < 'a , str > { if self . attrs . rename_format_string . is_none () { if let Some (remote_name) = self . serde_attrs . remote () . and_then (| r | r . segments . last ()) { return remote_name . ident . to_string () . into () ; } } self . serde_attrs . name () . deserialize_name () . into () } }
};
}
