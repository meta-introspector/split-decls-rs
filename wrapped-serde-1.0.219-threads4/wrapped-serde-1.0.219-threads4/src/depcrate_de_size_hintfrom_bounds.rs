// Generated macro for from_bounds (function)
macro_rules! Depcrate_de_size_hintfrom_bounds {
() => {
// Module: crate::de::size_hint
// Provides: {"from_bounds"}
// Dependencies: {}
pub fn from_bounds < I > (iter : & I) -> Option < usize > where I : Iterator , { helper (iter . size_hint ()) }
};
}
