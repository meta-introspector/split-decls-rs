// Generated macro for impl_903 (impl)
macro_rules! Depcrate_locale_likely_subtagsimpl_903 {
() => {
// Module: crate::locale::likely_subtags
// Provides: {"impl_903"}
// Dependencies: {}
impl DataProvider < LocaleLikelySubtagsScriptRegionV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < LocaleLikelySubtagsScriptRegionV1 > , DataError > { self . check_req :: < LocaleLikelySubtagsScriptRegionV1 > (req) ? ; let resources = LikelySubtagsResources :: try_from_cldr_cache (self . cldr () ?) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (transform (resources . get_common ()) . as_script_region ()) , }) } }
};
}
