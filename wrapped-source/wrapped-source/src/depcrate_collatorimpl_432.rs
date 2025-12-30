// Generated macro for impl_432 (impl)
macro_rules! Depcrate_collatorimpl_432 {
() => {
// Module: crate::collator
// Provides: {"impl_432"}
// Dependencies: {}
impl TryInto < CollationJamo < 'static > > for & collator_serde :: CollationJamo { type Error = DataError ; fn try_into (self) -> Result < CollationJamo < 'static > , Self :: Error > { Ok (CollationJamo { ce32s : ZeroVec :: alloc_from_slice (& self . ce32s) , }) } }
};
}
