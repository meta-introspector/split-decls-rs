// Generated macro for can_detect_truncation_of_all_tls12_handshake_payloads (function)
macro_rules! Depcrate_msgs_handshake_testcan_detect_truncation_of_all_tls12_handshake_payloads {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"can_detect_truncation_of_all_tls12_handshake_payloads"}
// Dependencies: {}
# [test] fn can_detect_truncation_of_all_tls12_handshake_payloads () { for hm in all_tls12_handshake_payloads () . iter () { let mut enc = hm . get_encoding () ; println ! ("test {hm:?} enc {enc:?}") ; for l in 0 .. enc . len () { assert ! (HandshakeMessagePayload :: read_bytes (& enc [.. l]) . is_err ()) } for l in 0 .. enc . len () - 4 { put_u24 (l as u32 , & mut enc [1 ..]) ; println ! ("  check len {l:?} enc {enc:?}") ; match (hm . 0 . handshake_type () , l) { (HandshakeType :: ClientHello , 41) | (HandshakeType :: ServerHello , 38) | (HandshakeType :: ServerKeyExchange , _) | (HandshakeType :: ClientKeyExchange , _) | (HandshakeType :: Finished , _) | (HandshakeType :: Unknown (_) , _) => continue , _ => { } } ; assert ! (HandshakeMessagePayload :: read_version (& mut Reader :: init (& enc) , ProtocolVersion :: TLSv1_2) . is_err ()) ; assert ! (HandshakeMessagePayload :: read_bytes (& enc) . is_err ()) ; } } }
};
}
