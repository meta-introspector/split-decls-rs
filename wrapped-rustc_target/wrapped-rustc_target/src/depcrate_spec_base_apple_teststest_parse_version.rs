// Generated macro for test_parse_version (function)
macro_rules! Depcrate_spec_base_apple_teststest_parse_version {
() => {
// Module: crate::spec::base::apple::tests
// Provides: {"test_parse_version"}
// Dependencies: {}
# [test] fn test_parse_version () { assert_eq ! ("10" . parse () , Ok (OSVersion :: new (10 , 0 , 0))) ; assert_eq ! ("10.12" . parse () , Ok (OSVersion :: new (10 , 12 , 0))) ; assert_eq ! ("10.12.6" . parse () , Ok (OSVersion :: new (10 , 12 , 6))) ; assert_eq ! ("9999.99.99" . parse () , Ok (OSVersion :: new (9999 , 99 , 99))) ; }
};
}
