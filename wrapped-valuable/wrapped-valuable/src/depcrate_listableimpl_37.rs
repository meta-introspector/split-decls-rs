// Generated macro for impl_37 (impl)
macro_rules! Depcrate_listableimpl_37 {
() => {
// Module: crate::listable
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T : Valuable > Valuable for alloc :: collections :: VecDeque < T > { fn as_value (& self) -> Value < '_ > { Value :: Listable (self as & dyn Listable) } fn visit (& self , visit : & mut dyn Visit) { let (first , second) = self . as_slices () ; T :: visit_slice (first , visit) ; T :: visit_slice (second , visit) ; } }
};
}
