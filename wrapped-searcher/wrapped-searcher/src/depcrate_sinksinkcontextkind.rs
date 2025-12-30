// Generated macro for SinkContextKind (enum)
macro_rules! Depcrate_sinkSinkContextKind {
() => {
// Module: crate::sink
// Provides: {"SinkContextKind"}
// Dependencies: {}
# [doc = " The type of context reported by a searcher."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum SinkContextKind { # [doc = " The line reported occurred before a match."] Before , # [doc = " The line reported occurred after a match."] After , # [doc = " Any other type of context reported, e.g., as a result of a searcher's"] # [doc = " \"passthru\" mode."] Other , }
};
}
