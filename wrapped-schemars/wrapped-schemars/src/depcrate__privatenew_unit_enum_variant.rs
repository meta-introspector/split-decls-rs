// Generated macro for new_unit_enum_variant (function)
macro_rules! Depcrate__privatenew_unit_enum_variant {
() => {
// Module: crate::_private
// Provides: {"new_unit_enum_variant"}
// Dependencies: {}
# [doc = " Create a schema for a unit enum variant"] # [must_use] pub fn new_unit_enum_variant (variant : & str) -> Schema { json_schema ! ({ "type" : "string" , "const" : variant , }) }
};
}
