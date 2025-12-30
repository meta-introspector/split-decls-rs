// Generated macro for impl_984 (impl)
macro_rules! Depcrate_personnames_person_names_format_data_providersimpl_984 {
() => {
// Module: crate::personnames::person_names_format_data_providers
// Provides: {"impl_984"}
// Dependencies: {}
impl DataProvider < PersonNamesFormatV1 > for crate :: SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < PersonNamesFormatV1 > , DataError > { let data : & Resource = self . cldr () ? . personnames () . read_and_parse (req . id . locale , "personNames.json") ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (PersonNamesFormat :: try_from (data) . map_err (| e | { DataError :: custom ("data for PersonNamesFormattingDefinition") . with_display_context (& e) }) ?) , }) } }
};
}
