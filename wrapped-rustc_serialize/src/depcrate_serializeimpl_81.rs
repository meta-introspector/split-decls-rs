// Generated macro for impl_81 (impl)
macro_rules! Depcrate_serializeimpl_81 {
() => {
// Module: crate::serialize
// Provides: {"impl_81"}
// Dependencies: {}
impl < D : Decoder , K , V , S > Decodable < D > for indexmap :: IndexMap < K , V , S > where K : Decodable < D > + Hash + Eq , V : Decodable < D > , S : BuildHasher + Default , { fn decode (d : & mut D) -> indexmap :: IndexMap < K , V , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | (Decodable :: decode (d) , Decodable :: decode (d))) . collect () } }
};
}
