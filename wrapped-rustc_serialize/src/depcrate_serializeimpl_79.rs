// Generated macro for impl_79 (impl)
macro_rules! Depcrate_serializeimpl_79 {
() => {
// Module: crate::serialize
// Provides: {"impl_79"}
// Dependencies: {}
impl < D : Decoder , T , S > Decodable < D > for HashSet < T , S > where T : Decodable < D > + Hash + Eq , S : BuildHasher + Default , { fn decode (d : & mut D) -> HashSet < T , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
};
}
