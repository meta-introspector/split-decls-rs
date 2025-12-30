// Generated macro for impl_83 (impl)
macro_rules! Depcrate_recent_blockhashesimpl_83 {
() => {
// Module: crate::recent_blockhashes
// Provides: {"impl_83"}
// Dependencies: {}
impl < T : Ord > Iterator for IntoIterSorted < T > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . inner . pop () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let exact = self . inner . len () ; (exact , Some (exact)) } }
};
}
