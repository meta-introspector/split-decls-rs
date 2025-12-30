// Generated macro for known (function)
macro_rules! Depcrate_rejectknown {
() => {
// Module: crate::reject
// Provides: {"known"}
// Dependencies: {}
pub (crate) fn known < T : Into < Known > > (err : T) -> Rejection { Rejection :: known (err . into ()) }
};
}
