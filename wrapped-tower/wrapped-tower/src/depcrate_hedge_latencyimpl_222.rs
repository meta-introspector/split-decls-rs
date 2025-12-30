// Generated macro for impl_222 (impl)
macro_rules! Depcrate_hedge_latencyimpl_222 {
() => {
// Module: crate::hedge::latency
// Provides: {"impl_222"}
// Dependencies: {}
impl < S , R > Latency < R , S > where R : Record + Clone , { pub const fn new < Request > (rec : R , service : S) -> Self where S : Service < Request > , S :: Error : Into < crate :: BoxError > , { Latency { rec , service } } }
};
}
