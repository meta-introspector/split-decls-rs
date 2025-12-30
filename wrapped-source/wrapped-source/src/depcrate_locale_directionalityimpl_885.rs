// Generated macro for impl_885 (impl)
macro_rules! Depcrate_locale_directionalityimpl_885 {
() => {
// Module: crate::locale::directionality
// Provides: {"impl_885"}
// Dependencies: {}
impl DataProvider < LocaleScriptDirectionV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < LocaleScriptDirectionV1 > , DataError > { self . check_req :: < LocaleScriptDirectionV1 > (req) ? ; let data : & cldr_serde :: directionality :: Resource = self . cldr () ? . core () . read_and_parse ("scriptMetadata.json") ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (ScriptDirection :: from (data)) , }) } }
};
}
