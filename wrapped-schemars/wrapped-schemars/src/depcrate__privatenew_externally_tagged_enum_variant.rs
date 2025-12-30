// Generated macro for new_externally_tagged_enum_variant (function)
macro_rules! Depcrate__privatenew_externally_tagged_enum_variant {
() => {
// Module: crate::_private
// Provides: {"new_externally_tagged_enum_variant"}
// Dependencies: {}
# [doc = " Create a schema for an externally tagged enum variant"] # [allow (clippy :: needless_pass_by_value)] # [must_use] pub fn new_externally_tagged_enum_variant (variant : & str , sub_schema : Schema) -> Schema { json_schema ! ({ "type" : "object" , "properties" : { variant : sub_schema } , "required" : [variant] , "additionalProperties" : false , }) }
};
}
