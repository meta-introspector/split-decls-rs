// Generated macro for impl_184 (impl)
macro_rules! Depcrate_json_schema_impls_bytes1impl_184 {
() => {
// Module: crate::json_schema_impls::bytes1
// Provides: {"impl_184"}
// Dependencies: {}
impl JsonSchema for bytes1 :: Bytes { fn schema_name () -> Cow < 'static , str > { "Bytes" . into () } fn schema_id () -> Cow < 'static , str > { "bytes::Bytes" . into () } fn json_schema (generator : & mut crate :: SchemaGenerator) -> crate :: Schema { let ty = match generator . contract () { Contract :: Deserialize => Value :: Array (vec ! ["array" . into () , "string" . into ()]) , Contract :: Serialize => "array" . into () , } ; let mut result = Schema :: default () ; result . insert ("type" . to_owned () , ty) ; result . insert ("items" . to_owned () , generator . subschema_for :: < u8 > () . into ()) ; result } }
};
}
