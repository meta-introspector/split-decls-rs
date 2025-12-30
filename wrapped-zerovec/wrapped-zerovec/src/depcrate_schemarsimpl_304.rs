// Generated macro for impl_304 (impl)
macro_rules! Depcrate_schemarsimpl_304 {
() => {
// Module: crate::schemars
// Provides: {"impl_304"}
// Dependencies: {}
impl < T : VarULE + JsonSchema + ? Sized , F : VarZeroVecFormat > JsonSchema for VarZeroVec < '_ , T , F > { fn inline_schema () -> bool { true } fn schema_name () -> Cow < 'static , str > { format ! ("VarZeroVec<{}>" , T :: schema_name ()) . into () } fn json_schema (generator : & mut schemars :: SchemaGenerator) -> schemars :: Schema { schemars :: json_schema ! ({ "type" : "array" , "items" : generator . subschema_for ::< T > () , }) } }
};
}
