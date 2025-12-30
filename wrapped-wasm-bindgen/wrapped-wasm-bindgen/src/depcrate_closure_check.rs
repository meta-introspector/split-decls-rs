// Generated macro for _check (function)
macro_rules! Depcrate_closure_check {
() => {
// Module: crate::closure
// Provides: {"_check"}
// Dependencies: {}
fn _check () { fn _assert < T : IntoWasmAbi > () { } _assert :: < & Closure < dyn Fn () > > () ; _assert :: < & Closure < dyn Fn (String) > > () ; _assert :: < & Closure < dyn Fn () -> String > > () ; _assert :: < & Closure < dyn FnMut () > > () ; _assert :: < & Closure < dyn FnMut (String) > > () ; _assert :: < & Closure < dyn FnMut () -> String > > () ; }
};
}
