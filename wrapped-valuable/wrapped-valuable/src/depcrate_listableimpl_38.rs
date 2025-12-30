// Generated macro for impl_38 (impl)
macro_rules! Depcrate_listableimpl_38 {
() => {
// Module: crate::listable
// Provides: {"impl_38"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T : Valuable > Listable for alloc :: collections :: VecDeque < T > { fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
