// Generated macro for impl_760 (impl)
macro_rules! Depcrate_ule_vartupleimpl_760 {
() => {
// Module: crate::ule::vartuple
// Provides: {"impl_760"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < A , V > alloc :: borrow :: ToOwned for VarTupleULE < A , V > where A : AsULE + 'static , V : VarULE + ? Sized , { type Owned = alloc :: boxed :: Box < Self > ; fn to_owned (& self) -> Self :: Owned { crate :: ule :: encode_varule_to_box (self) } }
};
}
