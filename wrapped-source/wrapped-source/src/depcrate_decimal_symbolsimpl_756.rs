// Generated macro for impl_756 (impl)
macro_rules! Depcrate_decimal_symbolsimpl_756 {
() => {
// Module: crate::decimal::symbols
// Provides: {"impl_756"}
// Dependencies: {}
impl IterableDataProviderCached < DecimalSymbolsV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { self . iter_ids_for_numbers_with_locales () } }
};
}
