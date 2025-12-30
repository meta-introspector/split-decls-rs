// Generated macro for impl_210 (impl)
macro_rules! Depcrate_hedge_delayimpl_210 {
() => {
// Module: crate::hedge::delay
// Provides: {"impl_210"}
// Dependencies: {}
impl < P , S > Delay < P , S > { pub const fn new < Request > (policy : P , service : S) -> Self where P : Policy < Request > , S : Service < Request > + Clone , S :: Error : Into < crate :: BoxError > , { Delay { policy , service } } }
};
}
