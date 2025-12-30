// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl < W > Drop for FlushGuard < W > where W : Write + 'static , { fn drop (& mut self) { match self . flush () { Ok (_) => () , Err (e) => e . report () , } } }
};
}
