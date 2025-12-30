// Generated macro for exemplar_chars_impls (macro)
macro_rules! Depcrate_charactersexemplar_chars_impls {
() => {
// Module: crate::characters
// Provides: {"exemplar_chars_impls"}
// Dependencies: {}
macro_rules ! exemplar_chars_impls { ($ data_marker_name : ident , $ cldr_serde_field_name : ident) => { impl DataProvider <$ data_marker_name > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ data_marker_name >, DataError > { self . check_req ::<$ data_marker_name > (req) ?; let data : & cldr_serde :: exemplar_chars :: Resource = self . cldr () ? . misc () . read_and_parse (req . id . locale , "characters.json") ?; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (string_to_prop_unicodeset (data . main . value . characters .$ cldr_serde_field_name . as_deref () . unwrap_or ("[]") ,)) , }) } } impl IterableDataProviderCached <$ data_marker_name > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { Ok (self . cldr () ? . misc () . list_locales () ? . map (DataIdentifierCow :: from_locale) . collect ()) } } } ; }
};
}
