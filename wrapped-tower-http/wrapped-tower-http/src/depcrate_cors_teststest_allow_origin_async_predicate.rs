// Generated macro for test_allow_origin_async_predicate (function)
macro_rules! Depcrate_cors_teststest_allow_origin_async_predicate {
() => {
// Module: crate::cors::tests
// Provides: {"test_allow_origin_async_predicate"}
// Dependencies: {}
# [tokio :: test] async fn test_allow_origin_async_predicate () { # [derive (Clone)] struct Client ; impl Client { async fn fetch_allowed_origins_for_path (& self , _path : String) -> Vec < HeaderValue > { vec ! [HeaderValue :: from_static ("http://example.com")] } } let client = Client ; let allow_origin = AllowOrigin :: async_predicate (| origin , parts | { let path = parts . uri . path () . to_owned () ; async move { let origins = client . fetch_allowed_origins_for_path (path) . await ; origins . contains (& origin) } }) ; let valid_origin = HeaderValue :: from_static ("http://example.com") ; let parts = http :: Request :: new ("hello world") . into_parts () . 0 ; let header = allow_origin . to_future (Some (& valid_origin) , & parts) . await . unwrap () ; assert_eq ! (header . 0 , header :: ACCESS_CONTROL_ALLOW_ORIGIN) ; assert_eq ! (header . 1 , valid_origin) ; let invalid_origin = HeaderValue :: from_static ("http://example.org") ; let parts = http :: Request :: new ("hello world") . into_parts () . 0 ; let res = allow_origin . to_future (Some (& invalid_origin) , & parts) . await ; assert ! (res . is_none ()) ; }
};
}
