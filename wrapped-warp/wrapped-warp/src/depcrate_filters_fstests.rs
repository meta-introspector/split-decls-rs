// Generated macro for tests (module)
macro_rules! Depcrate_filters_fstests {
() => {
// Module: crate::filters::fs
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: sanitize_path ; use bytes :: BytesMut ; # [test] fn test_sanitize_path () { let base = "/var/www" ; fn p (s : & str) -> & :: std :: path :: Path { s . as_ref () } assert_eq ! (sanitize_path (base , "/foo.html") . unwrap () , p ("/var/www/foo.html")) ; sanitize_path (base , "/../foo.html") . expect_err ("dot dot") ; sanitize_path (base , "/C:\\/foo.html") . expect_err ("C:\\") ; } # [test] fn test_reserve_at_least () { let mut buf = BytesMut :: new () ; let cap = 8_192 ; assert_eq ! (buf . len () , 0) ; assert_eq ! (buf . capacity () , 0) ; super :: reserve_at_least (& mut buf , cap) ; assert_eq ! (buf . len () , 0) ; assert_eq ! (buf . capacity () , cap) ; } }
};
}
