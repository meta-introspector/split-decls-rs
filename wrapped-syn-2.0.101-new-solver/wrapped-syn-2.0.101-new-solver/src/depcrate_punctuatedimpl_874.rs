// Generated macro for impl_874 (impl)
macro_rules! Depcrate_punctuatedimpl_874 {
() => {
// Module: crate::punctuated
// Provides: {"impl_874"}
// Dependencies: {}
impl < 'a , T , P > Iterator for Pairs < 'a , T , P > { type Item = Pair < & 'a T , & 'a P > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (t , p) | Pair :: Punctuated (t , p)) . or_else (| | self . last . next () . map (Pair :: End)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
