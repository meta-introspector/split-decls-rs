// Generated macro for impl_873 (impl)
macro_rules! Depcrate_locale_aliasesimpl_873 {
() => {
// Module: crate::locale::aliases
// Provides: {"impl_873"}
// Dependencies: {}
impl DataProvider < LocaleAliasesV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < LocaleAliasesV1 > , DataError > { self . check_req :: < LocaleAliasesV1 > (req) ? ; let data : & cldr_serde :: aliases :: Resource = self . cldr () ? . core () . read_and_parse ("supplemental/aliases.json") ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (Aliases :: from (data)) , }) } }
};
}
