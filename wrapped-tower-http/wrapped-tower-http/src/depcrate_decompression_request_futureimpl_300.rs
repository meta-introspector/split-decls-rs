// Generated macro for impl_300 (impl)
macro_rules! Depcrate_decompression_request_futureimpl_300 {
() => {
// Module: crate::decompression::request::future
// Provides: {"impl_300"}
// Dependencies: {}
impl < F , B , E > RequestDecompressionFuture < F , B , E > where F : Future < Output = Result < Response < B > , E > > , B : Body , { # [must_use] pub (super) fn unsupported_encoding (accept : AcceptEncoding) -> Self { Self { kind : Kind :: Unsupported { accept } , } } # [must_use] pub (super) fn inner (fut : F) -> Self { Self { kind : Kind :: Inner { fut } , } } }
};
}
