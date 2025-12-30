// Generated macro for impl_830 (impl)
macro_rules! Depcrate_displaynames_variantimpl_830 {
() => {
// Module: crate::displaynames::variant
// Provides: {"impl_830"}
// Dependencies: {}
impl IterableDataProviderCached < VariantDisplayNamesV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . displaynames () . list_locales () ? . filter (| locale | { self . cldr () . unwrap () . displaynames () . file_exists (locale , "variants.json") . unwrap_or_default () }) . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
