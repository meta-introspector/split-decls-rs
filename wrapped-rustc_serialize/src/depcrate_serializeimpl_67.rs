// Generated macro for impl_67 (impl)
macro_rules! Depcrate_serializeimpl_67 {
() => {
// Module: crate::serialize
// Provides: {"impl_67"}
// Dependencies: {}
impl < D : Decoder , A : Array < Item : Decodable < D > > > Decodable < D > for SmallVec < A > { fn decode (d : & mut D) -> SmallVec < A > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
};
}
