// Generated macro for impl_468 (impl)
macro_rules! Depcrate_ser_errorimpl_468 {
() => {
// Module: crate::ser::error
// Provides: {"impl_468"}
// Dependencies: {}
impl From < Error > for crate :: TomlError { fn from (e : Error) -> Self { Self :: custom (e . to_string () , None) } }
};
}
