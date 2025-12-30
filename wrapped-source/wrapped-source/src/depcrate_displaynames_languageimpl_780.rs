// Generated macro for impl_780 (impl)
macro_rules! Depcrate_displaynames_languageimpl_780 {
() => {
// Module: crate::displaynames::language
// Provides: {"impl_780"}
// Dependencies: {}
impl DataProvider < LocaleDisplayNamesV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < LocaleDisplayNamesV1 > , DataError > { self . check_req :: < LocaleDisplayNamesV1 > (req) ? ; let data : & cldr_serde :: displaynames :: language :: Resource = self . cldr () ? . displaynames () . read_and_parse (req . id . locale , "languages.json") ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (LocaleDisplayNames :: from (data)) , }) } }
};
}
