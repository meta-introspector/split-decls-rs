// Generated macro for decimal_impl (macro)
macro_rules! Depcrate_json_schema_impls_decimaldecimal_impl {
() => {
// Module: crate::json_schema_impls::decimal
// Provides: {"decimal_impl"}
// Dependencies: {}
macro_rules ! decimal_impl { ($ type : ty) => { impl JsonSchema for $ type { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { "Decimal" . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { let (ty , pattern) = match generator . contract () { Contract :: Deserialize => (Value :: Array (vec ! ["string" . into () , "number" . into ()]) , r"^-?\d+(\.\d+)?([eE]\d+)?$" . into () ,) , Contract :: Serialize => ("string" . into () , r"^-?\d+(\.\d+)?$" . into ()) , } ; let mut result = Schema :: default () ; result . insert ("type" . to_owned () , ty) ; result . insert ("pattern" . to_owned () , pattern) ; result } } } ; }
};
}
