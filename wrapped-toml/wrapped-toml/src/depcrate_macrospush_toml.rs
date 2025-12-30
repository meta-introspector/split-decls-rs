// Generated macro for push_toml (function)
macro_rules! Depcrate_macrospush_toml {
() => {
// Module: crate::macros
// Provides: {"push_toml"}
// Dependencies: {}
pub fn push_toml (root : & mut Value , path : & [& str]) { let target = traverse (root , path) ; if ! target . is_array () { * target = Value :: Array (Array :: new ()) ; } target . as_array_mut () . unwrap () . push (Value :: Table (Table :: new ())) ; }
};
}
