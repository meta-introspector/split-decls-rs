// Generated macro for process_serde_variant_attrs (function)
macro_rules! Depcrate_attr_schemars_to_serdeprocess_serde_variant_attrs {
() => {
// Module: crate::attr::schemars_to_serde
// Provides: {"process_serde_variant_attrs"}
// Dependencies: {}
fn process_serde_variant_attrs < 'a > (ctxt : & Ctxt , variants : impl Iterator < Item = & 'a mut Variant >) { for v in variants { process_attrs (ctxt , & mut v . attrs) ; process_serde_field_attrs (ctxt , v . fields . iter_mut ()) ; } }
};
}
