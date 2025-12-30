// Generated macro for impl_89 (impl)
macro_rules! Depcrate_spanimpl_89 {
() => {
// Module: crate::span
// Provides: {"impl_89"}
// Dependencies: {}
impl < 'a > nom :: InputIter for ParseSpan < 'a > { type Item = char ; type Iter = CharIndices < 'a > ; type IterElem = Chars < 'a > ; # [inline] fn iter_indices (& self) -> Self :: Iter { self . fragment . iter_indices () } # [inline] fn iter_elements (& self) -> Self :: IterElem { self . fragment . iter_elements () } # [inline] fn position < P > (& self , predicate : P) -> Option < usize > where P : Fn (Self :: Item) -> bool , { self . fragment . position (predicate) } # [inline] fn slice_index (& self , count : usize) -> Result < usize , nom :: Needed > { self . fragment . slice_index (count) } }
};
}
