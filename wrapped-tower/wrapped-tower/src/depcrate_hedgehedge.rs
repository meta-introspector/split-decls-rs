// Generated macro for Hedge (struct)
macro_rules! Depcrate_hedgeHedge {
() => {
// Module: crate::hedge
// Provides: {"Hedge"}
// Dependencies: {}
# [doc = " A middleware that pre-emptively retries requests which have been outstanding"] # [doc = " for longer than a given latency percentile.  If either of the original"] # [doc = " future or the retry future completes, that value is used."] # [derive (Debug)] pub struct Hedge < S , P > (Service < S , P >) ;
};
}
