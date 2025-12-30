// Generated macro for ReactorBridgeSinkError (enum)
macro_rules! Depcrate_reactor_bridgeReactorBridgeSinkError {
() => {
// Module: crate::reactor::bridge
// Provides: {"ReactorBridgeSinkError"}
// Dependencies: {}
# [doc = " An error type for bridge sink."] # [derive (Error , Clone , PartialEq , Eq , Debug)] pub enum ReactorBridgeSinkError { # [doc = " A bridge is an RAII Guard, it can only be closed by dropping the value."] # [error ("attempting to close the bridge via the sink")] AttemptClosure , }
};
}
