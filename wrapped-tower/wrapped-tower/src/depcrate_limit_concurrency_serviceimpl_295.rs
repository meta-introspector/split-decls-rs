// Generated macro for impl_295 (impl)
macro_rules! Depcrate_limit_concurrency_serviceimpl_295 {
() => {
// Module: crate::limit::concurrency::service
// Provides: {"impl_295"}
// Dependencies: {}
impl < T : Clone > Clone for ConcurrencyLimit < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , semaphore : self . semaphore . clone () , permit : None , } } }
};
}
