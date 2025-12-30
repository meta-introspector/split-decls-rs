// Generated macro for impl_431 (impl)
macro_rules! Depcrate_collatorimpl_431 {
() => {
// Module: crate::collator
// Provides: {"impl_431"}
// Dependencies: {}
impl TryInto < CollationDiacritics < 'static > > for & collator_serde :: CollationDiacritics { type Error = DataError ; fn try_into (self) -> Result < CollationDiacritics < 'static > , Self :: Error > { Ok (CollationDiacritics { secondaries : ZeroVec :: alloc_from_slice (& self . secondaries) , }) } }
};
}
