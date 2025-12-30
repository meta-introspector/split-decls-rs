// Generated macro for TryWait (trait)
macro_rules! Depcrate_async_helperTryWait {
() => {
// Module: crate::async_helper
// Provides: {"TryWait"}
// Dependencies: {}
# [doc = " [`TryWait`] allows [`AsyncWait`] to be used in synchronous methods."] pub (crate) trait TryWait { # [doc = " Registers the [`Pager`] in the [`Lock`], or synchronously waits for the [`Lock`] to be"] # [doc = " available."] fn try_wait (& mut self , lock : & Lock) ; }
};
}
