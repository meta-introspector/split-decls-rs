// Generated macro for impl_782 (impl)
macro_rules! Depcrate_displaynames_languageimpl_782 {
() => {
// Module: crate::displaynames::language
// Provides: {"impl_782"}
// Dependencies: {}
impl IterableDataProviderCached < LocaleDisplayNamesV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . displaynames () . list_locales () ? . filter (| locale | { self . cldr () . unwrap () . displaynames () . file_exists (locale , "languages.json") . unwrap_or_default () }) . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
