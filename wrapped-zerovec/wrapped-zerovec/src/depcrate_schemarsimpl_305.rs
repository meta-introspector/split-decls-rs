// Generated macro for impl_305 (impl)
macro_rules! Depcrate_schemarsimpl_305 {
() => {
// Module: crate::schemars
// Provides: {"impl_305"}
// Dependencies: {}
impl < 'a , T : AsULE + JsonSchema > JsonSchema for ZeroVec < 'a , T > { fn inline_schema () -> bool { true } fn schema_name () -> Cow < 'static , str > { alloc :: format ! ("ZeroVec<{}>" , T :: schema_name ()) . into () } fn json_schema (generator : & mut schemars :: SchemaGenerator) -> schemars :: Schema { schemars :: json_schema ! ({ "type" : "array" , "items" : generator . subschema_for ::< T > () , }) } }
};
}
