// Generated macro for ReactorScope (struct)
macro_rules! Depcrate_reactor_scopeReactorScope {
() => {
// Module: crate::reactor::scope
// Provides: {"ReactorScope"}
// Dependencies: {}
# [doc = " A handle to communicate with bridges."] pub struct ReactorScope < I , O > { input_stream : Pin < Box < dyn FusedStream < Item = I > > > , output_sink : Pin < Box < dyn Sink < O , Error = Infallible > > > , }
};
}
