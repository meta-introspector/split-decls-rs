// Generated macro for impl_920 (impl)
macro_rules! Depcrate_locale_parentsimpl_920 {
() => {
// Module: crate::locale::parents
// Provides: {"impl_920"}
// Dependencies: {}
impl DataProvider < LocaleParentsV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < LocaleParentsV1 > , DataError > { self . check_req :: < LocaleParentsV1 > (req) ? ; let parents_data : & cldr_serde :: parent_locales :: Resource = self . cldr () ? . core () . read_and_parse ("supplemental/parentLocales.json") ? ; let metadata = DataResponseMetadata :: default () ; Ok (DataResponse { metadata , payload : DataPayload :: from_owned (parents_data . into ()) , }) } }
};
}
