// Generated macro for impl_83 (impl)
macro_rules! Depcrate_serializeimpl_83 {
() => {
// Module: crate::serialize
// Provides: {"impl_83"}
// Dependencies: {}
impl < D : Decoder , T , S > Decodable < D > for indexmap :: IndexSet < T , S > where T : Decodable < D > + Hash + Eq , S : BuildHasher + Default , { fn decode (d : & mut D) -> indexmap :: IndexSet < T , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
};
}
