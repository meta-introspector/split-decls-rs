// Generated macro for impl_508 (impl)
macro_rules! Depcrate_currency_patternsimpl_508 {
() => {
// Module: crate::currency::patterns
// Provides: {"impl_508"}
// Dependencies: {}
impl IterableDataProviderCached < CurrencyPatternsDataV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . numbers () . list_locales () ? . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
