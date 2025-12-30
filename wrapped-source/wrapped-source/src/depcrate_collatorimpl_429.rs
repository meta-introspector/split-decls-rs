// Generated macro for impl_429 (impl)
macro_rules! Depcrate_collatorimpl_429 {
() => {
// Module: crate::collator
// Provides: {"impl_429"}
// Dependencies: {}
impl IterableDataProviderCached < CollationTailoringV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . list_ids ("_data") ? . into_iter () . filter (| s | * s != Default :: default ()) . collect ()) } }
};
}
