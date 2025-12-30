// Generated macro for path_value (function)
macro_rules! Depcrate_modulespath_value {
() => {
// Module: crate::modules
// Provides: {"path_value"}
// Dependencies: {}
fn path_value (attr : & ast :: Attribute) -> Option < Symbol > { if attr . has_name (sym :: path) { attr . value_str () } else { None } }
};
}
