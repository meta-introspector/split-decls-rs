// Generated macro for setter_throws (function)
macro_rules! Depcrate_utilsetter_throws {
() => {
// Module: crate::util
// Provides: {"setter_throws"}
// Dependencies: {}
# [doc = " Whether a setter is marked as throwing."] pub fn setter_throws (parent_js_name : & str , js_name : & str , attrs : & Option < ExtendedAttributeList > ,) -> bool { if let Some (parent) = BREAKING_SETTER_THROWS . get (parent_js_name) { if parent . contains (& js_name) { return false ; } } has_named_attribute (attrs . as_ref () , "SetterThrows") }
};
}
