// Generated macro for test_parse_size (function)
macro_rules! Depcrate_configtest_parse_size {
() => {
// Module: crate::config
// Provides: {"test_parse_size"}
// Dependencies: {}
# [test] fn test_parse_size () { assert_eq ! (None , parse_size ("")) ; assert_eq ! (None , parse_size ("bogus value")) ; assert_eq ! (Some (100) , parse_size ("100")) ; assert_eq ! (Some (2048) , parse_size ("2K")) ; assert_eq ! (Some (2048) , parse_size ("2k")) ; assert_eq ! (Some (10 * 1024 * 1024) , parse_size ("10M")) ; assert_eq ! (Some (TEN_GIGS) , parse_size ("10G")) ; assert_eq ! (Some (1024 * TEN_GIGS) , parse_size ("10T")) ; }
};
}
