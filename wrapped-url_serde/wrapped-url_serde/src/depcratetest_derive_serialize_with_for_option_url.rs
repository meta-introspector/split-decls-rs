// Generated macro for test_derive_serialize_with_for_option_url (function)
macro_rules! Depcratetest_derive_serialize_with_for_option_url {
() => {
// Module: crate
// Provides: {"test_derive_serialize_with_for_option_url"}
// Dependencies: {}
# [test] fn test_derive_serialize_with_for_option_url () { # [derive (Serialize , Debug , Eq , PartialEq)] struct Test { # [serde (serialize_with = "serialize" , rename = "_url_")] url : Option < Url > } let url_str = "http://www.test.com/foo/bar?$param=bazz" ; let expected = format ! (r#"{{"_url_":"{}"}}"# , url_str) ; let input = Test { url : Some (Url :: parse (url_str) . unwrap ()) } ; let got = serde_json :: to_string (& input) . unwrap () ; assert_eq ! (expected , got) ; let expected = format ! (r#"{{"_url_":null}}"#) ; let input = Test { url : None } ; let got = serde_json :: to_string (& input) . unwrap () ; assert_eq ! (expected , got) ; }
};
}
