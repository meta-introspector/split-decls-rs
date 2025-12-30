// Generated macro for ReactorScoped (trait)
macro_rules! Depcrate_reactor_scopeReactorScoped {
() => {
// Module: crate::reactor::scope
// Provides: {"ReactorScoped"}
// Dependencies: {}
# [doc = " A helper trait to extract the input and output type from a [ReactorStream]."] pub trait ReactorScoped : Stream + FusedStream { # [doc = " The Input Message."] type Input ; # [doc = " The Output Message."] type Output ; # [doc = " Creates a ReactorReceiver."] fn new < IS , OS > (input_stream : IS , output_sink : OS) -> Self where IS : Stream < Item = Self :: Input > + FusedStream + 'static , OS : Sink < Self :: Output , Error = Infallible > + 'static ; }
};
}
