// Generated macro for DefaultCallsite (struct)
macro_rules! Depcrate_callsiteDefaultCallsite {
() => {
// Module: crate::callsite
// Provides: {"DefaultCallsite"}
// Dependencies: {}
# [doc = " A default [`Callsite`] implementation."] # [derive (Debug)] pub struct DefaultCallsite { interest : AtomicU8 , registration : AtomicU8 , meta : & 'static Metadata < 'static > , next : AtomicPtr < Self > , }
};
}
