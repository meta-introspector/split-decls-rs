// Generated macro for json_schema_for_internally_tagged_enum_newtype_variant (function)
macro_rules! Depcrate__privatejson_schema_for_internally_tagged_enum_newtype_variant {
() => {
// Module: crate::_private
// Provides: {"json_schema_for_internally_tagged_enum_newtype_variant"}
// Dependencies: {}
pub fn json_schema_for_internally_tagged_enum_newtype_variant < T : ? Sized + JsonSchema > (generator : & mut SchemaGenerator ,) -> Schema { let mut schema = T :: json_schema (generator) ; let mut transform = AllowUnknownProperties :: default () ; transform_immediate_subschemas (& mut transform , & mut schema) ; if T :: inline_schema () || generator . settings () . inline_subschemas || schema . get ("type") . and_then (Value :: as_str) == Some ("null") || schema . get ("additionalProperties") . and_then (Value :: as_bool) == Some (false) || schema . get ("unevaluatedProperties") . and_then (Value :: as_bool) == Some (false) || transform . did_modify { return schema ; } generator . subschema_for :: < T > () }
};
}
