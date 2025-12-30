// Generated macro for find_path_value (function)
macro_rules! Depcrate_modulesfind_path_value {
() => {
// Module: crate::modules
// Provides: {"find_path_value"}
// Dependencies: {}
fn find_path_value (attrs : & [ast :: Attribute]) -> Option < Symbol > { attrs . iter () . flat_map (path_value) . next () }
};
}
