// Generated macro for impl_77 (impl)
macro_rules! Depcrate_serializeimpl_77 {
() => {
// Module: crate::serialize
// Provides: {"impl_77"}
// Dependencies: {}
impl < D : Decoder , K , V , S > Decodable < D > for HashMap < K , V , S > where K : Decodable < D > + Hash + Eq , V : Decodable < D > , S : BuildHasher + Default , { fn decode (d : & mut D) -> HashMap < K , V , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | (Decodable :: decode (d) , Decodable :: decode (d))) . collect () } }
};
}
