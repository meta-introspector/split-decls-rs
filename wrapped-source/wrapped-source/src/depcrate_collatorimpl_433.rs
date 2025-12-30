// Generated macro for impl_433 (impl)
macro_rules! Depcrate_collatorimpl_433 {
() => {
// Module: crate::collator
// Provides: {"impl_433"}
// Dependencies: {}
impl TryInto < CollationMetadata > for & collator_serde :: CollationMetadata { type Error = DataError ; fn try_into (self) -> Result < CollationMetadata , Self :: Error > { Ok (CollationMetadata { bits : self . bits }) } }
};
}
