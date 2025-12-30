// Generated macro for collation_provider (macro)
macro_rules! Depcrate_collatorcollation_provider {
() => {
// Module: crate::collator
// Provides: {"collation_provider"}
// Dependencies: {}
macro_rules ! collation_provider { ($ (($ marker : ident , $ serde_struct : ident , $ suffix : literal ,) ,) +) => { $ (impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . check_req ::<$ marker > (req) ?; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (self . load_toml ::< collator_serde ::$ serde_struct > (req . id , $ suffix) . and_then (TryInto :: try_into) . map_err (| e | e . with_req (<$ marker >:: INFO , req)) ?) , }) } } impl IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { self . list_ids ($ suffix) } }) + } ; }
};
}
