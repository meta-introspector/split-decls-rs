// Generated macro for impl_785 (impl)
macro_rules! Depcrate_util_call_all_unorderedimpl_785 {
() => {
// Module: crate::util::call_all::unordered
// Provides: {"impl_785"}
// Dependencies: {}
impl < Svc , S > Stream for CallAllUnordered < Svc , S > where Svc : Service < S :: Item > , S : Stream , { type Item = Result < Svc :: Response , Svc :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . project () . inner . poll_next (cx) } }
};
}
