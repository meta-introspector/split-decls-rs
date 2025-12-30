// Generated macro for macro_489 (macro)
macro_rules! Depcrate_ready_cache_cachemacro_489 {
() => {
// Module: crate::ready_cache::cache
// Provides: {"macro_489"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [doc = " A [`Future`] that becomes satisfied when an `S`-typed service is ready."] # [doc = ""] # [doc = " May fail due to cancelation, i.e. if the service is evicted from the balancer."] struct Pending < K , S , Req > { key : Option < K >, cancel : Option < CancelRx >, ready : Option < S >, _pd : std :: marker :: PhantomData < Req >, } }
};
}
