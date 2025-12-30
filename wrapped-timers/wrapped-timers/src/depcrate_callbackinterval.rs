// Generated macro for Interval (struct)
macro_rules! Depcrate_callbackInterval {
() => {
// Module: crate::callback
// Provides: {"Interval"}
// Dependencies: {}
# [doc = " A scheduled interval."] # [doc = ""] # [doc = " See `Interval::new` for scheduling new intervals."] # [doc = ""] # [doc = " Once scheduled, you can [`drop`] the [`Interval`] to clear it or [`forget`](Interval::forget) to leak it. Once forgotten, the interval will keep running forever."] # [doc = " This pattern is known as Resource Acquisition Is Initialization (RAII)."] # [derive (Debug)] # [must_use = "intervals cancel on drop; either call `forget` or `drop` explicitly"] pub struct Interval { id : Option < JsValue > , closure : Option < Closure < dyn FnMut () > > , }
};
}
