// Generated macro for array_impls (macro)
macro_rules! Depcrate_json_schema_impls_arrayarray_impls {
() => {
// Module: crate::json_schema_impls::array
// Provides: {"array_impls"}
// Dependencies: {}
macro_rules ! array_impls { ($ ($ len : tt) +) => { $ (impl < T : JsonSchema > JsonSchema for [T ; $ len] { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { format ! ("Array_size_{}_of_{}" , $ len , T :: schema_name ()) . into () } fn schema_id () -> Cow <'static , str > { format ! ("[{}; {}]" , $ len , T :: schema_id ()) . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "array" , "items" : serde_json :: Value :: from (generator . subschema_for ::< T > ()) , "minItems" : $ len , "maxItems" : $ len , }) } }) + } }
};
}
