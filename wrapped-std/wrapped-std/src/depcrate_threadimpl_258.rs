// Generated macro for impl_258 (impl)
macro_rules! Depcrate_threadimpl_258 {
() => {
// Module: crate::thread
// Provides: {"impl_258"}
// Dependencies: {}
impl Inner { fn parker (self : Pin < & Self >) -> Pin < & Parker > { unsafe { Pin :: map_unchecked (self , | inner | & inner . parker) } } }
};
}
