// Generated macro for impl_896 (impl)
macro_rules! Depcrate_punctuatedimpl_896 {
() => {
// Module: crate::punctuated
// Provides: {"impl_896"}
// Dependencies: {}
impl < 'a , T , P > Iterator for PairsMut < 'a , T , P > { type Item = Pair < & 'a mut T , & 'a mut P > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (t , p) | Pair :: Punctuated (t , p)) . or_else (| | self . last . next () . map (Pair :: End)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
