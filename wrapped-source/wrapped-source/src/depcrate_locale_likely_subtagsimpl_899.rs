// Generated macro for impl_899 (impl)
macro_rules! Depcrate_locale_likely_subtagsimpl_899 {
() => {
// Module: crate::locale::likely_subtags
// Provides: {"impl_899"}
// Dependencies: {}
impl DataProvider < LocaleLikelySubtagsExtendedV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < LocaleLikelySubtagsExtendedV1 > , DataError > { self . check_req :: < LocaleLikelySubtagsExtendedV1 > (req) ? ; let resources = LikelySubtagsResources :: try_from_cldr_cache (self . cldr () ?) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (transform (resources . get_extended ()) . as_extended ()) , }) } }
};
}
