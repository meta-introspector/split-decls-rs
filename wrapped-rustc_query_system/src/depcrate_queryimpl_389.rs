// Generated macro for impl_389 (impl)
macro_rules! Depcrate_queryimpl_389 {
() => {
// Module: crate::query
// Provides: {"impl_389"}
// Dependencies: {}
impl < 'tcx > QueryStackDeferred < 'tcx > { pub fn new < C : Copy + DynSync + DynSend + 'tcx > (context : C , extract : fn (C) -> QueryStackFrameExtra ,) -> Self { let extract : Arc < dyn Fn () -> QueryStackFrameExtra + DynSync + DynSend + 'tcx > = Arc :: new (move | | extract (context)) ; Self { _dummy : PhantomData , extract : unsafe { transmute (extract) } } } pub fn extract (& self) -> QueryStackFrameExtra { (self . extract) () } }
};
}
