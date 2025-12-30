// Generated macro for impl_75 (impl)
macro_rules! Depcrate_serializeimpl_75 {
() => {
// Module: crate::serialize
// Provides: {"impl_75"}
// Dependencies: {}
impl < D : Decoder , T > Decodable < D > for BTreeSet < T > where T : Decodable < D > + PartialEq + Ord , { fn decode (d : & mut D) -> BTreeSet < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
};
}
