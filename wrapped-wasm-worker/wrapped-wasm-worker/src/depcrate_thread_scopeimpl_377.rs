// Generated macro for impl_377 (impl)
macro_rules! Depcrate_thread_scopeimpl_377 {
() => {
// Module: crate::thread::scope
// Provides: {"impl_377"}
// Dependencies: {}
impl < 'scope , # [allow (single_use_lifetimes)] 'env > Scope < 'scope , 'env > { # [doc = " See [`std::thread::Scope::spawn()`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " See [`spawn()`](super::spawn())."] pub fn spawn < F , T > (& 'scope self , # [allow (clippy :: min_ident_chars)] f : F ,) -> ScopedJoinHandle < 'scope , T > where F : FnOnce () -> T + Send + 'scope , T : Send + 'scope , { Builder :: new () . spawn_scoped (self , f) . expect ("failed to spawn thread") } # [doc = " Implementation for"] # [doc = " [`ScopeExt::spawn_async()`](crate::web::ScopeExt::spawn_async)."] pub (crate) fn spawn_async_internal < F1 , F2 , T > (& 'scope self , task : F1 ,) -> ScopedJoinHandle < 'scope , T > where F1 : 'scope + FnOnce () -> F2 + Send , F2 : 'scope + Future < Output = T > , T : 'scope + Send , { Builder :: new () . spawn_scoped_async_internal (self , task) . expect ("failed to spawn thread") } # [doc = " Implementation for"] # [doc = " [`ScopeExt::spawn_async()`](crate::web::ScopeExt::spawn_async)."] # [cfg (feature = "message")] pub (crate) fn spawn_with_message_internal < F1 , F2 , T , M > (& 'scope self , task : F1 , message : M ,) -> ScopedJoinHandle < 'scope , T > where F1 : 'scope + FnOnce (M) -> F2 + Send , F2 : 'scope + Future < Output = T > , T : 'scope + Send , M : 'scope + MessageSend , { Builder :: new () . spawn_scoped_with_message_internal (self , task , message) . expect ("failed to spawn thread") } }
};
}
