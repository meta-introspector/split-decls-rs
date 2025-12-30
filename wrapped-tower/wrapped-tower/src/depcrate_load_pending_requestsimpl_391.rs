// Generated macro for impl_391 (impl)
macro_rules! Depcrate_load_pending_requestsimpl_391 {
() => {
// Module: crate::load::pending_requests
// Provides: {"impl_391"}
// Dependencies: {}
# [cfg (feature = "discover")] impl < D , C > PendingRequestsDiscover < D , C > { # [doc = " Wraps a [`Discover`], wrapping all of its services with [`PendingRequests`]."] pub const fn new < Request > (discover : D , completion : C) -> Self where D : Discover , D :: Service : Service < Request > , C : TrackCompletion < Handle , < D :: Service as Service < Request > > :: Response > , { Self { discover , completion , } } }
};
}
