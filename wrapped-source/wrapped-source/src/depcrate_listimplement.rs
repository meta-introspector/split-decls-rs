// Generated macro for implement (macro)
macro_rules! Depcrate_listimplement {
() => {
// Module: crate::list
// Provides: {"implement"}
// Dependencies: {}
macro_rules ! implement { ($ marker : ident) => { impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . check_req ::<$ marker > (req) ?; load (self , req) } } impl IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { Ok (self . cldr () ? . misc () . list_locales () ? . flat_map (| l | { [ListFormatterPatterns :: SHORT , ListFormatterPatterns :: NARROW , ListFormatterPatterns :: WIDE ,] . into_iter () . map (move | a | DataIdentifierCow :: from_borrowed_and_owned (a , l . clone ())) }) . collect ()) } } } ; }
};
}
