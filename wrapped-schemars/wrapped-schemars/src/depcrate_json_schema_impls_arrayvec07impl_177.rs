// Generated macro for impl_177 (impl)
macro_rules! Depcrate_json_schema_impls_arrayvec07impl_177 {
() => {
// Module: crate::json_schema_impls::arrayvec07
// Provides: {"impl_177"}
// Dependencies: {}
impl < T , const CAP : usize > JsonSchema for ArrayVec < T , CAP > where T : JsonSchema , { inline_schema ! () ; fn schema_name () -> alloc :: borrow :: Cow < 'static , str > { format ! ("Array_up_to_size_{}_of_{}" , CAP , T :: schema_name ()) . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "array" , "items" : generator . subschema_for ::< T > () , "maxItems" : CAP }) } }
};
}
