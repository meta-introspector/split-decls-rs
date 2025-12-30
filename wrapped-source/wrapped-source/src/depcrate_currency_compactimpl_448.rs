// Generated macro for impl_448 (impl)
macro_rules! Depcrate_currency_compactimpl_448 {
() => {
// Module: crate::currency::compact
// Provides: {"impl_448"}
// Dependencies: {}
impl IterableDataProviderCached < ShortCurrencyCompactV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . numbers () . list_locales () ? . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
