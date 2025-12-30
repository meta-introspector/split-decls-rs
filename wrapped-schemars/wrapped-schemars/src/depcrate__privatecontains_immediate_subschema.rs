// Generated macro for contains_immediate_subschema (function)
macro_rules! Depcrate__privatecontains_immediate_subschema {
() => {
// Module: crate::_private
// Provides: {"contains_immediate_subschema"}
// Dependencies: {}
fn contains_immediate_subschema (schema_obj : & Map < String , Value >) -> bool { ["if" , "allOf" , "anyOf" , "oneOf" , "$ref"] . into_iter () . any (| k | schema_obj . contains_key (k)) }
};
}
