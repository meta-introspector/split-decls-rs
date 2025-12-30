// Generated macro for set_impl (macro)
macro_rules! Depcrate_json_schema_impls_sequencesset_impl {
() => {
// Module: crate::json_schema_impls::sequences
// Provides: {"set_impl"}
// Dependencies: {}
macro_rules ! set_impl { ($ ($ desc : tt) +) => { impl $ ($ desc) + where T : JsonSchema , { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { format ! ("Set_of_{}" , T :: schema_name ()) . into () } fn schema_id () -> Cow <'static , str > { format ! ("Set<{}>" , T :: schema_id ()) . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "array" , "uniqueItems" : true , "items" : generator . subschema_for ::< T > () , }) } } } ; }
};
}
