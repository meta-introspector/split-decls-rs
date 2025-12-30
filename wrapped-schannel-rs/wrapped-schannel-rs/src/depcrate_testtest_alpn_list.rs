// Generated macro for test_alpn_list (function)
macro_rules! Depcrate_testtest_alpn_list {
() => {
// Module: crate::test
// Provides: {"test_alpn_list"}
// Dependencies: {}
# [test] fn test_alpn_list () { let raw_proto_alpn_list = b"\x02h2" ; let proto_list = & [& [2 , 0 , 0 , 0 , raw_proto_alpn_list . len () as u8 , 0] as & [u8] , raw_proto_alpn_list ,] . concat () ; let full_alpn_list = [& [proto_list . len () as u8 , 0 , 0 , 0] as & [u8] , proto_list] . concat () ; assert_eq ! (& AlpnList :: new (& [b"h2" . to_vec ()]) as & [u8] , & full_alpn_list as & [u8]) ; let raw_proto_alpn_list = b"\x02h2\x08http/1.1" ; let proto_list = & [& [2 , 0 , 0 , 0 , raw_proto_alpn_list . len () as u8 , 0] as & [u8] , raw_proto_alpn_list ,] . concat () ; let full_alpn_list = [& [proto_list . len () as u8 , 0 , 0 , 0] as & [u8] , proto_list] . concat () ; assert_eq ! (& AlpnList :: new (& [b"h2" . to_vec () , b"http/1.1" . to_vec ()]) as & [u8] , & full_alpn_list as & [u8]) ; }
};
}
