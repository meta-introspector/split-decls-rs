// Generated macro for impl_434 (impl)
macro_rules! Depcrate_collatorimpl_434 {
() => {
// Module: crate::collator
// Provides: {"impl_434"}
// Dependencies: {}
impl TryInto < CollationReordering < 'static > > for & collator_serde :: CollationReordering { type Error = DataError ; fn try_into (self) -> Result < CollationReordering < 'static > , Self :: Error > { Ok (CollationReordering { min_high_no_reorder : self . min_high_no_reorder , reorder_table : ZeroVec :: alloc_from_slice (& self . reorder_table) , reorder_ranges : ZeroVec :: alloc_from_slice (& self . reorder_ranges) , }) } }
};
}
