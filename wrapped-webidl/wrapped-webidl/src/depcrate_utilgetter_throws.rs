// Generated macro for getter_throws (function)
macro_rules! Depcrate_utilgetter_throws {
() => {
// Module: crate::util
// Provides: {"getter_throws"}
// Dependencies: {}
# [doc = " Whether a getter is marked as throwing."] pub fn getter_throws (parent_js_name : & str , js_name : & str , attrs : & Option < ExtendedAttributeList > ,) -> bool { if let Some (parent) = BREAKING_GETTER_THROWS . get (parent_js_name) { if parent . contains (& js_name) { return false ; } } has_named_attribute (attrs . as_ref () , "GetterThrows") }
};
}
