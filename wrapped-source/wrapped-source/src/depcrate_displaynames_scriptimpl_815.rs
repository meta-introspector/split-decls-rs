// Generated macro for impl_815 (impl)
macro_rules! Depcrate_displaynames_scriptimpl_815 {
() => {
// Module: crate::displaynames::script
// Provides: {"impl_815"}
// Dependencies: {}
impl IterableDataProviderCached < ScriptDisplayNamesV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . displaynames () . list_locales () ? . filter (| locale | { self . cldr () . unwrap () . displaynames () . file_exists (locale , "scripts.json") . unwrap_or_default () }) . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
