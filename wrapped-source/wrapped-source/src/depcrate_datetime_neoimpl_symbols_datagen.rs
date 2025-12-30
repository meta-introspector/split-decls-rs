// Generated macro for impl_symbols_datagen (macro)
macro_rules! Depcrate_datetime_neoimpl_symbols_datagen {
() => {
// Module: crate::datetime::neo
// Provides: {"impl_symbols_datagen"}
// Dependencies: {}
macro_rules ! impl_symbols_datagen { ($ marker : ident , $ calendar : expr , $ lengths : ident , $ convert : expr) => { impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . load_neo_symbols_marker ::<$ marker > (req , $ calendar , $ convert) } } impl IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { self . iter_ids_neo ($ calendar , $ lengths) } } } ; }
};
}
