// Generated macro for impl_241 (impl)
macro_rules! Depcrate_async_helperimpl_241 {
() => {
// Module: crate::async_helper
// Provides: {"impl_241"}
// Dependencies: {}
impl AsyncWait { # [doc = " Awaits the [`Lock`] to be available."] # [inline] pub async fn wait (self : & mut Pin < & mut Self >) { let this = unsafe { ptr :: read (self) } ; let mut pinned_pager = unsafe { Pin :: new_unchecked (& mut this . get_unchecked_mut () . pager) } ; let _result = pinned_pager . poll_async () . await ; } }
};
}
