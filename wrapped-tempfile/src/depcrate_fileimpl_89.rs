// Generated macro for impl_89 (impl)
macro_rules! Depcrate_fileimpl_89 {
() => {
// Module: crate::file
// Provides: {"impl_89"}
// Dependencies: {}
impl < F > error :: Error for PersistError < F > { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { Some (& self . error) } }
};
}
