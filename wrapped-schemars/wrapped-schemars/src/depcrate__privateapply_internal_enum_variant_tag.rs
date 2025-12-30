// Generated macro for apply_internal_enum_variant_tag (function)
macro_rules! Depcrate__privateapply_internal_enum_variant_tag {
() => {
// Module: crate::_private
// Provides: {"apply_internal_enum_variant_tag"}
// Dependencies: {}
# [doc = " Update a schema for an internally tagged enum variant"] pub fn apply_internal_enum_variant_tag (schema : & mut Schema , tag_name : & str , variant : & str , deny_unknown_fields : bool ,) { let obj = schema . ensure_object () ; let is_unit = obj . get ("type") . and_then (Value :: as_str) == Some ("null") ; obj . insert ("type" . to_owned () , "object" . into ()) ; if let Some (properties) = obj . entry ("properties") . or_insert (Value :: Object (Map :: new ())) . as_object_mut () { properties . insert (tag_name . to_string () , json ! ({ "type" : "string" , "const" : variant }) ,) ; } if let Some (required) = obj . entry ("required") . or_insert (Value :: Array (Vec :: new ())) . as_array_mut () { required . insert (0 , tag_name . into ()) ; } if deny_unknown_fields && is_unit { obj . entry ("additionalProperties") . or_insert (false . into ()) ; } }
};
}
