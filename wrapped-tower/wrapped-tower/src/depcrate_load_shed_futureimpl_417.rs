// Generated macro for impl_417 (impl)
macro_rules! Depcrate_load_shed_futureimpl_417 {
() => {
// Module: crate::load_shed::future
// Provides: {"impl_417"}
// Dependencies: {}
impl < F > ResponseFuture < F > { pub (crate) fn called (fut : F) -> Self { ResponseFuture { state : ResponseState :: Called { fut } , } } pub (crate) fn overloaded () -> Self { ResponseFuture { state : ResponseState :: Overloaded , } } }
};
}
