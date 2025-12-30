// Generated macro for impl_774 (impl)
macro_rules! Depcrate_util_call_all_orderedimpl_774 {
() => {
// Module: crate::util::call_all::ordered
// Provides: {"impl_774"}
// Dependencies: {}
impl < Svc , S > Stream for CallAll < Svc , S > where Svc : Service < S :: Item > , S : Stream , { type Item = Result < Svc :: Response , Svc :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . project () . inner . poll_next (cx) } }
};
}
