// Generated macro for impl_neo_skeleton_datagen (macro)
macro_rules! Depcrate_datetime_neo_skeletonimpl_neo_skeleton_datagen {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"impl_neo_skeleton_datagen"}
// Dependencies: {}
macro_rules ! impl_neo_skeleton_datagen { ($ marker : ident , $ calendar : expr) => { impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . load_neo_skeletons_key (req , Some ($ calendar) , gen_date_components) } } impl IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { self . neo_date_skeleton_supported_locales ($ calendar) } } } ; }
};
}
