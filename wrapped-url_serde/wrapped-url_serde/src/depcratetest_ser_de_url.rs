// Generated macro for test_ser_de_url (function)
macro_rules! Depcratetest_ser_de_url {
() => {
// Module: crate
// Provides: {"test_ser_de_url"}
// Dependencies: {}
# [test] fn test_ser_de_url () { let url = Url :: parse ("http://www.test.com/foo/bar?$param=bazz") . unwrap () ; let s = serde_json :: to_string (& Ser :: new (& url)) . unwrap () ; let new_url : Url = serde_json :: from_str (& s) . map (De :: into_inner) . unwrap () ; assert_eq ! (url , new_url) ; }
};
}
