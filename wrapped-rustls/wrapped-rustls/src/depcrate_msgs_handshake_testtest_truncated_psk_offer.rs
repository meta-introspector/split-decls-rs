// Generated macro for test_truncated_psk_offer (function)
macro_rules! Depcrate_msgs_handshake_testtest_truncated_psk_offer {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"test_truncated_psk_offer"}
// Dependencies: {}
# [test] fn test_truncated_psk_offer () { let ext = PresharedKeyOffer { identities : vec ! [PresharedKeyIdentity :: new (vec ! [3 , 4 , 5] , 123456)] , binders : vec ! [PresharedKeyBinder :: from (vec ! [1 , 2 , 3])] , } ; let mut enc = ext . get_encoding () ; println ! ("testing {ext:?} enc {enc:?}") ; for l in 0 .. enc . len () { if l == 9 { continue ; } put_u16 (l as u16 , & mut enc) ; let rc = PresharedKeyOffer :: read_bytes (& enc) ; assert ! (rc . is_err ()) ; } }
};
}
