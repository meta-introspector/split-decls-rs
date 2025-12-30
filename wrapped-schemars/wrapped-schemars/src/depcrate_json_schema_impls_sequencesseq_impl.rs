// Generated macro for seq_impl (macro)
macro_rules! Depcrate_json_schema_impls_sequencesseq_impl {
() => {
// Module: crate::json_schema_impls::sequences
// Provides: {"seq_impl"}
// Dependencies: {}
macro_rules ! seq_impl { ($ ($ desc : tt) +) => { impl $ ($ desc) + where T : JsonSchema , { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { format ! ("Array_of_{}" , T :: schema_name ()) . into () } fn schema_id () -> Cow <'static , str > { format ! ("[{}]" , T :: schema_id ()) . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "array" , "items" : generator . subschema_for ::< T > () , }) } } } ; }
};
}
