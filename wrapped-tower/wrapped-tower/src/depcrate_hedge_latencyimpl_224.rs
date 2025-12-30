// Generated macro for impl_224 (impl)
macro_rules! Depcrate_hedge_latencyimpl_224 {
() => {
// Module: crate::hedge::latency
// Provides: {"impl_224"}
// Dependencies: {}
impl < R , F , T , E > Future for ResponseFuture < R , F > where R : Record , F : Future < Output = Result < T , E > > , E : Into < crate :: BoxError > , { type Output = Result < T , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let rsp = ready ! (this . inner . poll (cx)) . map_err (Into :: into) ? ; let duration = Instant :: now () . saturating_duration_since (* this . start) ; this . rec . record (duration) ; Poll :: Ready (Ok (rsp)) } }
};
}
