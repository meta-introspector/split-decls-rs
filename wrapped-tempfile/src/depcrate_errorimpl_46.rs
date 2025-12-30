// Generated macro for impl_46 (impl)
macro_rules! Depcrate_errorimpl_46 {
() => {
// Module: crate::error
// Provides: {"impl_46"}
// Dependencies: {}
impl error :: Error for PathError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { self . err . source () } }
};
}
