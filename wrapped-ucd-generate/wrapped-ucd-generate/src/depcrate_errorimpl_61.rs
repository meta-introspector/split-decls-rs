// Generated macro for impl_61 (impl)
macro_rules! Depcrate_errorimpl_61 {
() => {
// Module: crate::error
// Provides: {"impl_61"}
// Dependencies: {}
impl From < ucd_trie :: Error > for Error { fn from (err : ucd_trie :: Error) -> Error { Error :: Other (err . to_string ()) } }
};
}
