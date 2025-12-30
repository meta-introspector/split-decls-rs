// Generated macro for impl_86 (impl)
macro_rules! Depcrate_buffer_futureimpl_86 {
() => {
// Module: crate::buffer::future
// Provides: {"impl_86"}
// Dependencies: {}
impl < T > ResponseFuture < T > { pub (crate) fn new (rx : message :: Rx < T >) -> Self { ResponseFuture { state : ResponseState :: Rx { rx } , } } pub (crate) fn failed (err : crate :: BoxError) -> Self { ResponseFuture { state : ResponseState :: Failed { error : Some (err) } , } } }
};
}
