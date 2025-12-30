// Generated macro for impl_130 (impl)
macro_rules! Depcrate_hygieneimpl_130 {
() => {
// Module: crate::hygiene
// Provides: {"impl_130"}
// Dependencies: {}
impl < D : SpanDecoder > Decodable < D > for LocalExpnId { fn decode (d : & mut D) -> Self { ExpnId :: expect_local (ExpnId :: decode (d)) } }
};
}
