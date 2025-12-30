// Generated macro for test_normalize_canonicalize_iana_coverage (function)
macro_rules! Depcrate_time_zones_namestest_normalize_canonicalize_iana_coverage {
() => {
// Module: crate::time_zones::names
// Provides: {"test_normalize_canonicalize_iana_coverage"}
// Dependencies: {}
# [doc = " Tests that all IANA time zone IDs normalize and canonicalize to their correct form."] # [test] fn test_normalize_canonicalize_iana_coverage () { let provider = crate :: SourceDataProvider :: new_testing () ; let iana2bcp = provider . iana_to_bcp47_map () . unwrap () ; let bcp2iana = provider . bcp47_to_canonical_iana_map () . unwrap () ; let parser = icu :: time :: zone :: iana :: IanaParserExtended :: try_new_unstable (& provider) . unwrap () ; let parser = parser . as_borrowed () ; for (iana_id , bcp47) in iana2bcp { let unnormalized = iana_id . to_ascii_uppercase () ; let r = parser . parse (& unnormalized) ; assert_eq ! (r . time_zone , * bcp47) ; assert_eq ! (r . canonical , bcp2iana . get (bcp47) . unwrap ()) ; assert_eq ! (r . normalized , iana_id) ; } }
};
}
