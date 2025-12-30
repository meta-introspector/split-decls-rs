// Generated macro for impl_69 (impl)
macro_rules! Depcrate_serializeimpl_69 {
() => {
// Module: crate::serialize
// Provides: {"impl_69"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for ThinVec < T > { fn decode (d : & mut D) -> ThinVec < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
};
}
