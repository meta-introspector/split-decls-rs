// Generated macro for make_data_provider (macro)
macro_rules! Depcrate_relativetimemake_data_provider {
() => {
// Module: crate::relativetime
// Provides: {"make_data_provider"}
// Dependencies: {}
macro_rules ! make_data_provider { ($ ($ marker : ident) ,+ $ (,) ?) => { $ (impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . check_req ::<$ marker > (req) ?; let resource : & cldr_serde :: date_fields :: Resource = self . cldr () ? . dates ("gregorian") . read_and_parse (req . id . locale , "dateFields.json") ?; let fields = & resource . main . value . dates . fields ; let field = marker_filters () . get (&$ marker :: INFO) . ok_or (DataErrorKind :: MarkerNotFound . into_error ()) ?; let data = fields . 0 . get (* field) . ok_or (DataError :: custom ("Field not found in relative time format data." ,)) ?; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (RelativeTimePatternData { relatives : data . relatives . iter () . map (| r | (& r . count , r . pattern . as_ref ())) . collect () , past : (& data . past) . into () , future : (& data . future) . into () , }) , }) } } impl IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { Ok (self . cldr () ? . dates ("gregorian") . list_locales () ? . map (DataIdentifierCow :: from_locale) . collect ()) } }) + } ; }
};
}
