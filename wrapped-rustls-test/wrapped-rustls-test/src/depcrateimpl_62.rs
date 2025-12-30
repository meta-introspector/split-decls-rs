// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl Drop for TestNonBlockIo { fn drop (& mut self) { assert ! (self . reads . is_empty ()) ; assert ! (self . writes . is_empty ()) ; } }
};
}
