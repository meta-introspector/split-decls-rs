// Generated macro for impl_units_display_names_provider (macro)
macro_rules! Depcrate_units_categorized_display_nameimpl_units_display_names_provider {
() => {
// Module: crate::units::categorized_display_name
// Provides: {"impl_units_display_names_provider"}
// Dependencies: {}
macro_rules ! impl_units_display_names_provider { ($ marker : ty , $ unit_type : expr , $ category : expr) => { impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . check_req ::<$ marker > (req) ?; self . get_display_name_payload ::<$ marker > (req) } } impl crate :: IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { self . get_display_name_iter_ids_cached ($ unit_type , $ category) } } } ; }
};
}
