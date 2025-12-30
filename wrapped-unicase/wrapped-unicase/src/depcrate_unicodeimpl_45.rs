// Generated macro for impl_45 (impl)
macro_rules! Depcrate_unicodeimpl_45 {
() => {
// Module: crate::unicode
// Provides: {"impl_45"}
// Dependencies: {}
impl < S : AsRef < str > > Unicode < S > { pub fn to_folded_case (& self) -> String { self . 0 . as_ref () . chars () . flat_map (lookup) . collect () } }
};
}
