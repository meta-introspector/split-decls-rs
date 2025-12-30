// Generated macro for impl_307 (impl)
macro_rules! Depcrate_schemarsimpl_307 {
() => {
// Module: crate::schemars
// Provides: {"impl_307"}
// Dependencies: {}
impl < 'a , K , V > JsonSchema for ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized + JsonSchema , V : ZeroMapKV < 'a > + ? Sized + JsonSchema , { fn inline_schema () -> bool { true } fn schema_name () -> Cow < 'static , str > { format ! ("ZeroMap<{}, {}>" , K :: schema_name () , V :: schema_name ()) . into () } fn json_schema (generator : & mut schemars :: SchemaGenerator) -> schemars :: Schema { < alloc :: collections :: BTreeMap < & K , & V > as JsonSchema > :: json_schema (generator) } }
};
}
