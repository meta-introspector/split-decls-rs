// Generated macro for impl_485 (impl)
macro_rules! Depcrate_currency_essentialsimpl_485 {
() => {
// Module: crate::currency::essentials
// Provides: {"impl_485"}
// Dependencies: {}
impl IterableDataProviderCached < CurrencyEssentialsV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . numbers () . list_locales () ? . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
