// Generated macro for impl_891 (impl)
macro_rules! Depcrate_punctuatedimpl_891 {
() => {
// Module: crate::punctuated
// Provides: {"impl_891"}
// Dependencies: {}
impl < 'a , T , P > Iterator for Pairs < 'a , T , P > { type Item = Pair < & 'a T , & 'a P > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (t , p) | Pair :: Punctuated (t , p)) . or_else (| | self . last . next () . map (Pair :: End)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
