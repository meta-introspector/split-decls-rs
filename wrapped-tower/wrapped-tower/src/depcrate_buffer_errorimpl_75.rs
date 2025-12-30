// Generated macro for impl_75 (impl)
macro_rules! Depcrate_buffer_errorimpl_75 {
() => {
// Module: crate::buffer::error
// Provides: {"impl_75"}
// Dependencies: {}
impl std :: error :: Error for ServiceError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { Some (& * * self . inner) } }
};
}
