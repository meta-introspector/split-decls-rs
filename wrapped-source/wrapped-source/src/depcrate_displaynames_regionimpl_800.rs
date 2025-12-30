// Generated macro for impl_800 (impl)
macro_rules! Depcrate_displaynames_regionimpl_800 {
() => {
// Module: crate::displaynames::region
// Provides: {"impl_800"}
// Dependencies: {}
impl IterableDataProviderCached < RegionDisplayNamesV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . displaynames () . list_locales () ? . filter (| locale | { self . cldr () . unwrap () . displaynames () . file_exists (locale , "territories.json") . unwrap_or_default () }) . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
