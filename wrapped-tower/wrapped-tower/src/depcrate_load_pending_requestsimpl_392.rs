// Generated macro for impl_392 (impl)
macro_rules! Depcrate_load_pending_requestsimpl_392 {
() => {
// Module: crate::load::pending_requests
// Provides: {"impl_392"}
// Dependencies: {}
# [cfg (feature = "discover")] impl < D , C > Stream for PendingRequestsDiscover < D , C > where D : Discover , C : Clone , { type Item = Result < Change < D :: Key , PendingRequests < D :: Service , C > > , D :: Error > ; # [doc = " Yields the next discovery change set."] fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { use self :: Change :: * ; let this = self . project () ; let change = match ready ! (this . discover . poll_discover (cx)) . transpose () ? { None => return Poll :: Ready (None) , Some (Insert (k , svc)) => Insert (k , PendingRequests :: new (svc , this . completion . clone ())) , Some (Remove (k)) => Remove (k) , } ; Poll :: Ready (Some (Ok (change))) } }
};
}
