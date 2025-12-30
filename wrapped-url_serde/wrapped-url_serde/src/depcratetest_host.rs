// Generated macro for test_host (function)
macro_rules! Depcratetest_host {
() => {
// Module: crate
// Provides: {"test_host"}
// Dependencies: {}
# [test] fn test_host () { for host in & [Host :: Domain ("foo.com" . to_owned ()) , Host :: Ipv4 ("127.0.0.1" . parse () . unwrap ()) , Host :: Ipv6 ("::1" . parse () . unwrap ()) ,] { let json = serde_json :: to_string (& Ser (host)) . unwrap () ; let de : De < Host > = serde_json :: from_str (& json) . unwrap () ; assert_eq ! (de . into_inner () , * host) } }
};
}
