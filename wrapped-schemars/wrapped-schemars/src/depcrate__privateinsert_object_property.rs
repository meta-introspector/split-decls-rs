// Generated macro for insert_object_property (function)
macro_rules! Depcrate__privateinsert_object_property {
() => {
// Module: crate::_private
// Provides: {"insert_object_property"}
// Dependencies: {}
pub fn insert_object_property (schema : & mut Schema , key : & str , is_optional : bool , sub_schema : Schema ,) { let obj = schema . ensure_object () ; if let Some (properties) = obj . entry ("properties") . or_insert (Value :: Object (Map :: new ())) . as_object_mut () { properties . insert (key . to_owned () , sub_schema . into ()) ; } if ! is_optional { if let Some (req) = obj . entry ("required") . or_insert (Value :: Array (Vec :: new ())) . as_array_mut () { req . push (key . into ()) ; } } }
};
}
