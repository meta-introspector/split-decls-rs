// Generated macro for insert_validation_property (function)
macro_rules! Depcrate__privateinsert_validation_property {
() => {
// Module: crate::_private
// Provides: {"insert_validation_property"}
// Dependencies: {}
pub fn insert_validation_property (schema : & mut Schema , required_type : & str , key : & str , value : impl Into < Value > ,) { if schema . has_type (required_type) || (required_type == "number" && schema . has_type ("integer")) { schema . insert (key . to_owned () , value . into ()) ; } }
};
}
