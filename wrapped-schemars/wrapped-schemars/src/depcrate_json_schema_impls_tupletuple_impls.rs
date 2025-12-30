// Generated macro for tuple_impls (macro)
macro_rules! Depcrate_json_schema_impls_tupletuple_impls {
() => {
// Module: crate::json_schema_impls::tuple
// Provides: {"tuple_impls"}
// Dependencies: {}
macro_rules ! tuple_impls { ($ ($ len : expr => ($ ($ name : ident) +)) +) => { $ (impl <$ ($ name : JsonSchema) ,+> JsonSchema for ($ ($ name ,) +) { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { let mut name = "Tuple_of_" . to_owned () ; name . push_str (& [$ ($ name :: schema_name ()) ,+] . join ("_and_")) ; name . into () } fn schema_id () -> Cow <'static , str > { let mut id = "(" . to_owned () ; id . push_str (& [$ ($ name :: schema_id ()) ,+] . join (",")) ; id . push (')') ; id . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "array" , "prefixItems" : [$ (generator . subschema_for ::<$ name > ()) ,+] , "minItems" : $ len , "maxItems" : $ len , }) } }) + } }
};
}
