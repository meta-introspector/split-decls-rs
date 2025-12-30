// Generated macro for process_serde_field_attrs (function)
macro_rules! Depcrate_attr_schemars_to_serdeprocess_serde_field_attrs {
() => {
// Module: crate::attr::schemars_to_serde
// Provides: {"process_serde_field_attrs"}
// Dependencies: {}
fn process_serde_field_attrs < 'a > (ctxt : & Ctxt , fields : impl Iterator < Item = & 'a mut Field >) { for f in fields { process_attrs (ctxt , & mut f . attrs) ; } }
};
}
