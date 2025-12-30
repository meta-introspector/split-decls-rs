// Generated macro for impl_368 (impl)
macro_rules! Depcrate_load_peak_ewmaimpl_368 {
() => {
// Module: crate::load::peak_ewma
// Provides: {"impl_368"}
// Dependencies: {}
# [cfg (feature = "discover")] impl < D , C > Stream for PeakEwmaDiscover < D , C > where D : Discover , C : Clone , { type Item = Result < Change < D :: Key , PeakEwma < D :: Service , C > > , D :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; let change = match ready ! (this . discover . poll_discover (cx)) . transpose () ? { None => return Poll :: Ready (None) , Some (Change :: Remove (k)) => Change :: Remove (k) , Some (Change :: Insert (k , svc)) => { let peak_ewma = PeakEwma :: new (svc , * this . default_rtt , * this . decay_ns , this . completion . clone () ,) ; Change :: Insert (k , peak_ewma) } } ; Poll :: Ready (Some (Ok (change))) } }
};
}
