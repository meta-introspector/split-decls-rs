// Generated macro for impl_iterable_data_provider (macro)
macro_rules! Depcrate_time_zonesimpl_iterable_data_provider {
() => {
// Module: crate::time_zones
// Provides: {"impl_iterable_data_provider"}
// Dependencies: {}
macro_rules ! impl_iterable_data_provider { ($ ($ marker : ident) ,+) => { $ (impl IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { Ok (self . cldr () ? . dates ("gregorian") . list_locales () ? . map (DataIdentifierCow :: from_locale) . collect ()) } }) + } ; }
};
}
