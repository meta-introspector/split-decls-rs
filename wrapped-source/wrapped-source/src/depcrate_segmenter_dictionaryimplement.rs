// Generated macro for implement (macro)
macro_rules! Depcrate_segmenter_dictionaryimplement {
() => {
// Module: crate::segmenter::dictionary
// Provides: {"implement"}
// Dependencies: {}
macro_rules ! implement { ($ marker : ident , [$ ($ supported : expr) ,*]) => { impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . check_req ::<$ marker > (req) ?; let data = self . load_dictionary_data (req) ?; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data) , }) } } impl IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self ,) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { const SUPPORTED : & [& DataMarkerAttributes] = & [$ (DataMarkerAttributes :: from_str_or_panic ($ supported)) ,*] ; Ok (SUPPORTED . iter () . copied () . map (DataIdentifierCow :: from_marker_attributes) . collect ()) } } } ; }
};
}
