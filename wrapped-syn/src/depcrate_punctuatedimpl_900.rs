// Generated macro for impl_900 (impl)
macro_rules! Depcrate_punctuatedimpl_900 {
() => {
// Module: crate::punctuated
// Provides: {"impl_900"}
// Dependencies: {}
impl < T , P > Iterator for IntoPairs < T , P > { type Item = Pair < T , P > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (t , p) | Pair :: Punctuated (t , p)) . or_else (| | self . last . next () . map (Pair :: End)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
