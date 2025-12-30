// Generated macro for impl_344 (impl)
macro_rules! Depcrate_load_constantimpl_344 {
() => {
// Module: crate::load::constant
// Provides: {"impl_344"}
// Dependencies: {}
# [doc = " Proxies [`Discover`] such that all changes are wrapped with a constant load."] # [cfg (feature = "discover")] impl < D : Discover + Unpin , M : Copy > Stream for Constant < D , M > { type Item = Result < Change < D :: Key , Constant < D :: Service , M > > , D :: Error > ; # [doc = " Yields the next discovery change set."] fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { use self :: Change :: * ; let this = self . project () ; let change = match ready ! (Pin :: new (this . inner) . poll_discover (cx)) . transpose () ? { None => return Poll :: Ready (None) , Some (Insert (k , svc)) => Insert (k , Constant :: new (svc , * this . load)) , Some (Remove (k)) => Remove (k) , } ; Poll :: Ready (Some (Ok (change))) } }
};
}
