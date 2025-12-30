// Generated macro for impl_750 (impl)
macro_rules! Depcrate_metrics_in_flight_requestsimpl_750 {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"impl_750"}
// Dependencies: {}
impl < F , B , E > Future for ResponseFuture < F > where F : Future < Output = Result < Response < B > , E > > , { type Output = Result < Response < ResponseBody < B > > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let response = ready ! (this . inner . poll (cx)) ? ; let guard = this . guard . take () . unwrap () ; let response = response . map (move | body | ResponseBody { inner : body , guard }) ; Poll :: Ready (Ok (response)) } }
};
}
