// Generated macro for impl_139 (impl)
macro_rules! Depcrateimpl_139 {
() => {
// Module: crate
// Provides: {"impl_139"}
// Dependencies: {}
impl Iterator for CharIter { type Item = char ; fn next (& mut self) -> Option < char > { match self . 0 { CharIterInternal :: Iterator (ref mut it) => it . next () . map (| c | cp_decode (* c)) , CharIterInternal :: Double (a , b) => { self . 0 = CharIterInternal :: Single (b) ; Some (a) } , CharIterInternal :: Single (c) => { self . 0 = CharIterInternal :: Exhausted ; Some (c) } , CharIterInternal :: Exhausted => None } } fn size_hint (& self) -> (usize , Option < usize >) { match self . 0 { CharIterInternal :: Iterator (ref it) => it . size_hint () , CharIterInternal :: Double (_ , _) => (2 , Some (2)) , CharIterInternal :: Single (_) => (1 , Some (1)) , CharIterInternal :: Exhausted => (0 , Some (0)) } } }
};
}
