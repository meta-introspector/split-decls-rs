// Generated macro for impl_985 (impl)
macro_rules! Depcrate_personnames_person_names_format_data_providersimpl_985 {
() => {
// Module: crate::personnames::person_names_format_data_providers
// Provides: {"impl_985"}
// Dependencies: {}
impl IterableDataProviderCached < PersonNamesFormatV1 > for crate :: SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . personnames () . list_locales () ? . filter (| locale | { self . cldr () . unwrap () . personnames () . file_exists (locale , "personNames.json") . unwrap_or_default () }) . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
