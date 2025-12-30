// Generated macro for impl_161 (impl)
macro_rules! Depcrate_filter_futureimpl_161 {
() => {
// Module: crate::filter::future
// Provides: {"impl_161"}
// Dependencies: {}
impl < P , S , Request > AsyncResponseFuture < P , S , Request > where P : AsyncPredicate < Request > , S : Service < P :: Request > , S :: Error : Into < BoxError > , { pub (crate) fn new (check : P :: Future , service : S) -> Self { Self { state : State :: Check { check } , service , } } }
};
}
