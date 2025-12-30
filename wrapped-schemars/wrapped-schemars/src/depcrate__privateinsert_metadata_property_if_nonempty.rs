// Generated macro for insert_metadata_property_if_nonempty (function)
macro_rules! Depcrate__privateinsert_metadata_property_if_nonempty {
() => {
// Module: crate::_private
// Provides: {"insert_metadata_property_if_nonempty"}
// Dependencies: {}
pub fn insert_metadata_property_if_nonempty (schema : & mut Schema , key : & str , value : impl Into < String > ,) { let value : String = value . into () ; if ! value . is_empty () { schema . insert (key . to_owned () , value . into ()) ; } }
};
}
