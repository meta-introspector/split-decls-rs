// Generated macro for impl_73 (impl)
macro_rules! Depcrate_buffer_errorimpl_73 {
() => {
// Module: crate::buffer::error
// Provides: {"impl_73"}
// Dependencies: {}
impl ServiceError { pub (crate) fn new (inner : BoxError) -> ServiceError { let inner = Arc :: new (inner) ; ServiceError { inner } } pub (crate) fn clone (& self) -> ServiceError { ServiceError { inner : self . inner . clone () , } } }
};
}
