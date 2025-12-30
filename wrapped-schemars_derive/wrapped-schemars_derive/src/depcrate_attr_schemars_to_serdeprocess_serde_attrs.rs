// Generated macro for process_serde_attrs (function)
macro_rules! Depcrate_attr_schemars_to_serdeprocess_serde_attrs {
() => {
// Module: crate::attr::schemars_to_serde
// Provides: {"process_serde_attrs"}
// Dependencies: {}
pub fn process_serde_attrs (input : & mut syn :: DeriveInput) -> syn :: Result < () > { let ctxt = Ctxt :: new () ; process_attrs (& ctxt , & mut input . attrs) ; match & mut input . data { Data :: Struct (s) => process_serde_field_attrs (& ctxt , s . fields . iter_mut ()) , Data :: Enum (e) => process_serde_variant_attrs (& ctxt , e . variants . iter_mut ()) , Data :: Union (u) => process_serde_field_attrs (& ctxt , u . fields . named . iter_mut ()) , } ctxt . check () }
};
}
