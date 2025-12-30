// Generated macro for normalise_additional_unevaluated_properties (function)
macro_rules! Depcrate__privatenormalise_additional_unevaluated_properties {
() => {
// Module: crate::_private
// Provides: {"normalise_additional_unevaluated_properties"}
// Dependencies: {}
fn normalise_additional_unevaluated_properties (schema_obj1 : & mut Map < String , Value > , schema_obj2 : & Map < String , Value > ,) { if schema_obj1 . contains_key ("additionalProperties") && (schema_obj2 . contains_key ("unevaluatedProperties") || contains_immediate_subschema (schema_obj2)) { let ap = schema_obj1 . remove ("additionalProperties") ; schema_obj1 . insert ("unevaluatedProperties" . to_owned () , ap . into ()) ; } }
};
}
