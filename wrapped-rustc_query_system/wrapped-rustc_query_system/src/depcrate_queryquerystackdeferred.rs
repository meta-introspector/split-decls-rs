// Generated macro for QueryStackDeferred (struct)
macro_rules! Depcrate_queryQueryStackDeferred {
() => {
// Module: crate::query
// Provides: {"QueryStackDeferred"}
// Dependencies: {}
# [doc = " Track a 'side effect' for a particular query."] # [doc = " This is used to hold a closure which can create `QueryStackFrameExtra`."] # [derive (Clone)] pub struct QueryStackDeferred < 'tcx > { _dummy : PhantomData < & 'tcx () > , extract : Arc < dyn Fn () -> QueryStackFrameExtra + DynSync + DynSend > , }
};
}
