// Generated macro for impl_779 (impl)
macro_rules! Depcrate_displaynames_languageimpl_779 {
() => {
// Module: crate::displaynames::language
// Provides: {"impl_779"}
// Dependencies: {}
impl DataProvider < LanguageDisplayNamesV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < LanguageDisplayNamesV1 > , DataError > { self . check_req :: < LanguageDisplayNamesV1 > (req) ? ; let data : & cldr_serde :: displaynames :: language :: Resource = self . cldr () ? . displaynames () . read_and_parse (req . id . locale , "languages.json") ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (LanguageDisplayNames :: from (data)) , }) } }
};
}
