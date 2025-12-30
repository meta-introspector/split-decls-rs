// Generated macro for impl_65 (impl)
macro_rules! Depcrateimpl_65 {
() => {
// Module: crate
// Provides: {"impl_65"}
// Dependencies: {}
impl < L : Iterator < Item = T > , R : Iterator < Item = T > , T , U : PartialEq , F : Fn (& T) -> U > Iterator for DifferenceIter < L , R , F > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { loop { let left = self . left . next () ; let right = self . right . next () ? ; if left . as_ref () . map (& self . compare) != Some ((self . compare) (& right)) { return Some (right) ; } } } }
};
}
