// Generated macro for impl_156 (impl)
macro_rules! Depcrate_eitherimpl_156 {
() => {
// Module: crate::either
// Provides: {"impl_156"}
// Dependencies: {}
impl < L , R > futures_core :: stream :: Stream for Either < L , R > where L : futures_core :: stream :: Stream , R : futures_core :: stream :: Stream < Item = L :: Item > , { type Item = L :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { delegate_call ! (self . poll_next (cx)) } }
};
}
