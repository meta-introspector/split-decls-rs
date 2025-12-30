// Generated macro for impl_18 (impl)
macro_rules! Depcrate_mock_futureimpl_18 {
() => {
// Module: crate::mock::future
// Provides: {"impl_18"}
// Dependencies: {}
impl < T > ResponseFuture < T > { pub (crate) fn new (rx : Rx < T >) -> ResponseFuture < T > { ResponseFuture { rx : Some (rx) } } pub (crate) fn closed () -> ResponseFuture < T > { ResponseFuture { rx : None } } }
};
}
