// Generated macro for impl_567 (impl)
macro_rules! Depcrate_retry_futureimpl_567 {
() => {
// Module: crate::retry::future
// Provides: {"impl_567"}
// Dependencies: {}
impl < P , S , Request > ResponseFuture < P , S , Request > where P : Policy < Request , S :: Response , S :: Error > , S : Service < Request > , { pub (crate) fn new (request : Option < Request > , retry : Retry < P , S > , future : S :: Future ,) -> ResponseFuture < P , S , Request > { ResponseFuture { request , retry , state : State :: Called { future } , } } }
};
}
