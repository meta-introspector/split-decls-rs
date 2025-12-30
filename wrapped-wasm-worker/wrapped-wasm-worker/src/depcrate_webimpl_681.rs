// Generated macro for impl_681 (impl)
macro_rules! Depcrate_webimpl_681 {
() => {
// Module: crate::web
// Provides: {"impl_681"}
// Dependencies: {}
impl BuilderExt for Builder { fn spawn_async < F1 , F2 , T > (self , # [allow (clippy :: min_ident_chars)] f : F1 ,) -> io :: Result < JoinHandle < T > > where F1 : 'static + FnOnce () -> F2 + Send , F2 : 'static + Future < Output = T > , T : 'static + Send , { self . spawn_async_internal (f) } # [cfg (any (feature = "message" , docsrs))] fn spawn_with_message < F1 , F2 , T , M > (self , # [allow (clippy :: min_ident_chars)] f : F1 , message : M ,) -> io :: Result < JoinHandle < T > > where F1 : 'static + FnOnce (M) -> F2 + Send , F2 : 'static + Future < Output = T > , T : 'static + Send , M : 'static + MessageSend , { self . spawn_with_message_internal (f , message) } fn spawn_scoped_async < 'scope , # [allow (single_use_lifetimes)] 'env , F1 , F2 , T > (self , scope : & 'scope Scope < 'scope , 'env > , # [allow (clippy :: min_ident_chars)] f : F1 ,) -> io :: Result < ScopedJoinHandle < 'scope , T > > where F1 : 'scope + FnOnce () -> F2 + Send , F2 : 'scope + Future < Output = T > , T : 'scope + Send , { self . spawn_scoped_async_internal (scope , f) } # [cfg (any (feature = "message" , docsrs))] fn spawn_scoped_with_message < 'scope , # [allow (single_use_lifetimes)] 'env , F1 , F2 , T , M > (self , scope : & 'scope Scope < 'scope , 'env > , # [allow (clippy :: min_ident_chars)] f : F1 , message : M ,) -> io :: Result < ScopedJoinHandle < 'scope , T > > where F1 : 'scope + FnOnce (M) -> F2 + Send , F2 : 'scope + Future < Output = T > , T : 'scope + Send , M : 'scope + MessageSend , { self . spawn_scoped_with_message_internal (scope , f , message) } }
};
}
