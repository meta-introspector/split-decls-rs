// Generated macro for WithFallback (struct)
macro_rules! Depcrate_processorWithFallback {
() => {
// Module: crate::processor
// Provides: {"WithFallback"}
// Dependencies: {}
# [doc = " A [`Processor`] composed of a primary and a fallback `Processor`."] # [doc = ""] # [doc = " This type is returned by [`Processor::or`]."] # [derive (Debug)] pub struct WithFallback < P , F > { primary : P , fallback : F , }
};
}
