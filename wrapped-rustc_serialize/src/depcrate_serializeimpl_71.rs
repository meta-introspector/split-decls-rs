// Generated macro for impl_71 (impl)
macro_rules! Depcrate_serializeimpl_71 {
() => {
// Module: crate::serialize
// Provides: {"impl_71"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for VecDeque < T > { fn decode (d : & mut D) -> VecDeque < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
};
}
