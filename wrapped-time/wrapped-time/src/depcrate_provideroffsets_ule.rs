// Generated macro for offsets_ule (function)
macro_rules! Depcrate_provideroffsets_ule {
() => {
// Module: crate::provider
// Provides: {"offsets_ule"}
// Dependencies: {}
# [test] fn offsets_ule () { # [track_caller] fn assert_round_trip (offset : UtcOffset) { let variants = VariantOffsets :: from_standard (offset) ; assert_eq ! (variants , VariantOffsets :: from_unaligned (VariantOffsets :: to_unaligned (variants))) ; } assert_round_trip (UtcOffset :: try_from_str ("+01:00") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("+01:15") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("+01:30") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("+01:45") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("+01:10") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("+01:20") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("+01:40") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("+01:50") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("-01:00") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("-01:15") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("-01:30") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("-01:45") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("-01:10") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("-01:20") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("-01:40") . unwrap ()) ; assert_round_trip (UtcOffset :: try_from_str ("-01:50") . unwrap ()) ; }
};
}
