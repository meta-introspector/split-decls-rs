// Generated macro for impl_41 (impl)
macro_rules! Depcrate_serializeimpl_41 {
() => {
// Module: crate::serialize
// Provides: {"impl_41"}
// Dependencies: {}
impl < D : Decoder , T : Decodable < D > > Decodable < D > for Vec < T > { default fn decode (d : & mut D) -> Vec < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
};
}
