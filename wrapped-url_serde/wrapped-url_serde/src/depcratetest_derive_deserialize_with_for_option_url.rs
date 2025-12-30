// Generated macro for test_derive_deserialize_with_for_option_url (function)
macro_rules! Depcratetest_derive_deserialize_with_for_option_url {
() => {
// Module: crate
// Provides: {"test_derive_deserialize_with_for_option_url"}
// Dependencies: {}
# [test] fn test_derive_deserialize_with_for_option_url () { # [derive (Deserialize , Debug , Eq , PartialEq)] struct Test { # [serde (deserialize_with = "deserialize" , rename = "_url_")] url : Option < Url > } let url_str = "http://www.test.com/foo/bar?$param=bazz" ; let expected = Test { url : Some (Url :: parse (url_str) . unwrap ()) } ; let json_string = format ! (r#"{{"_url_": "{}"}}"# , url_str) ; let got : Test = serde_json :: from_str (& json_string) . unwrap () ; assert_eq ! (expected , got) ; let expected = Test { url : None } ; let json_string = r#"{"_url_": null}"# ; let got : Test = serde_json :: from_str (& json_string) . unwrap () ; assert_eq ! (expected , got) ; }
};
}
