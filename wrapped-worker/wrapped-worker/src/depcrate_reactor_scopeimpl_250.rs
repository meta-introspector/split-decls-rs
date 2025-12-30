// Generated macro for impl_250 (impl)
macro_rules! Depcrate_reactor_scopeimpl_250 {
() => {
// Module: crate::reactor::scope
// Provides: {"impl_250"}
// Dependencies: {}
impl < I , O > ReactorScoped for ReactorScope < I , O > { type Input = I ; type Output = O ; # [inline] fn new < IS , OS > (input_stream : IS , output_sink : OS) -> Self where IS : Stream < Item = Self :: Input > + FusedStream + 'static , OS : Sink < Self :: Output , Error = Infallible > + 'static , { Self { input_stream : Box :: pin (input_stream) , output_sink : Box :: pin (output_sink) , } } }
};
}
