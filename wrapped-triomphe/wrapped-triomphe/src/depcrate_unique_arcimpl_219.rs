// Generated macro for impl_219 (impl)
macro_rules! Depcrate_unique_arcimpl_219 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_219"}
// Dependencies: {}
impl < A > FromIterator < A > for UniqueArc < [A] > { fn from_iter < T : IntoIterator < Item = A > > (iter : T) -> Self { let iter = iter . into_iter () ; let (lower , upper) = iter . size_hint () ; let arc : Arc < [A] > = if Some (lower) == upper { let iter = IteratorAsExactSizeIterator :: new (iter) ; Arc :: from_header_and_iter (() , iter) . into () } else { let vec = iter . collect :: < Vec < _ > > () ; Arc :: from (vec) } ; unsafe { UniqueArc :: from_arc (arc) } } }
};
}
