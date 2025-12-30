// Generated macro for impl_778 (impl)
macro_rules! Depcrate_cors_allow_headersimpl_778 {
() => {
// Module: crate::cors::allow_headers
// Provides: {"impl_778"}
// Dependencies: {}
impl fmt :: Debug for AllowHeaders { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { AllowHeadersInner :: Const (inner) => f . debug_tuple ("Const") . field (inner) . finish () , AllowHeadersInner :: MirrorRequest => f . debug_tuple ("MirrorRequest") . finish () , } } }
};
}
