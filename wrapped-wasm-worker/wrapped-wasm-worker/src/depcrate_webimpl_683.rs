// Generated macro for impl_683 (impl)
macro_rules! Depcrate_webimpl_683 {
() => {
// Module: crate::web
// Provides: {"impl_683"}
// Dependencies: {}
impl < 'scope > ScopeExt < 'scope > for Scope < 'scope , '_ > { fn spawn_async < F1 , F2 , T > (& 'scope self , # [allow (clippy :: min_ident_chars)] f : F1 ,) -> ScopedJoinHandle < 'scope , T > where F1 : 'scope + FnOnce () -> F2 + Send , F2 : 'scope + Future < Output = T > , T : 'scope + Send , { self . spawn_async_internal (f) } # [cfg (any (feature = "message" , docsrs))] fn spawn_with_message < F1 , F2 , T , M > (& 'scope self , # [allow (clippy :: min_ident_chars)] f : F1 , message : M ,) -> ScopedJoinHandle < 'scope , T > where F1 : 'scope + FnOnce (M) -> F2 + Send , F2 : 'scope + Future < Output = T > , T : 'scope + Send , M : 'scope + MessageSend , { self . spawn_with_message_internal (f , message) } }
};
}
