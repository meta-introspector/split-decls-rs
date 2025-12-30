// Generated macro for AsyncGuard (struct)
macro_rules! Depcrate_async_helperAsyncGuard {
() => {
// Module: crate::async_helper
// Provides: {"AsyncGuard"}
// Dependencies: {}
# [doc = " [`AsyncGuard`] is used when an asynchronous task needs to be suspended without invalidating any"] # [doc = " references."] # [doc = ""] # [doc = " The validity of those references must be checked and verified by the user."] # [derive (Debug , Default)] pub (crate) struct AsyncGuard { # [doc = " [`Guard`] that can be dropped without invalidating any references."] guard : UnsafeCell < Option < Guard > > , }
};
}
