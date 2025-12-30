// Generated macro for impl_968 (impl)
macro_rules! Depcrate_percentimpl_968 {
() => {
// Module: crate::percent
// Provides: {"impl_968"}
// Dependencies: {}
impl IterableDataProviderCached < PercentEssentialsV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . numbers () . list_locales () ? . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
