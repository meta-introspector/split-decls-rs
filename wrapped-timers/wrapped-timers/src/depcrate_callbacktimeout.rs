// Generated macro for Timeout (struct)
macro_rules! Depcrate_callbackTimeout {
() => {
// Module: crate::callback
// Provides: {"Timeout"}
// Dependencies: {}
# [doc = " A scheduled timeout."] # [doc = ""] # [doc = " See `Timeout::new` for scheduling new timeouts."] # [doc = ""] # [doc = " Once scheduled, you can [`drop`] the [`Timeout`] to clear it or [`forget`](Timeout::forget) to leak it. Once forgotten, the interval will keep running forever."] # [doc = " This pattern is known as Resource Acquisition Is Initialization (RAII)."] # [derive (Debug)] # [must_use = "timeouts cancel on drop; either call `forget` or `drop` explicitly"] pub struct Timeout { id : Option < JsValue > , closure : Option < Closure < dyn FnMut () > > , }
};
}
