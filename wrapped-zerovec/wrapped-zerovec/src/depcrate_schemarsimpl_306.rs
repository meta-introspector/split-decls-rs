// Generated macro for impl_306 (impl)
macro_rules! Depcrate_schemarsimpl_306 {
() => {
// Module: crate::schemars
// Provides: {"impl_306"}
// Dependencies: {}
impl < T : AsULE + JsonSchema > JsonSchema for ZeroSlice < T > { fn inline_schema () -> bool { true } fn schema_name () -> Cow < 'static , str > { format ! ("ZeroSlice<{}>" , T :: schema_name ()) . into () } fn json_schema (generator : & mut schemars :: SchemaGenerator) -> schemars :: Schema { schemars :: json_schema ! ({ "type" : "array" , "items" : generator . subschema_for ::< T > () , }) } }
};
}
