// Generated macro for implement (macro)
macro_rules! Depcrate_pluralsimplement {
() => {
// Module: crate::plurals
// Provides: {"implement"}
// Dependencies: {}
macro_rules ! implement { ($ marker : ident) => { impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . check_req ::<$ marker > (req) ?; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (PluralRulesData :: from (self . get_rules_for (<$ marker >:: INFO) ? . 0 . get (& icu :: locale :: LanguageIdentifier :: from ((req . id . locale . language , req . id . locale . script , req . id . locale . region ,))) . ok_or (DataErrorKind :: IdentifierNotFound . into_error ()) ?,)) , }) } } impl IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { Ok (self . get_rules_for (<$ marker >:: INFO) ? . 0 . keys () . map (| l | DataIdentifierCow :: from_locale (DataLocale :: from (l))) . collect ()) } } } ; }
};
}
