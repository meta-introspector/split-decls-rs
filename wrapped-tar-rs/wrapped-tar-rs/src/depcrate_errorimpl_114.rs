// Generated macro for impl_114 (impl)
macro_rules! Depcrate_errorimpl_114 {
() => {
// Module: crate::error
// Provides: {"impl_114"}
// Dependencies: {}
impl error :: Error for TarError { fn description (& self) -> & str { & self . desc } fn source (& self) -> Option < & (dyn error :: Error + 'static) > { Some (& self . io) } }
};
}
