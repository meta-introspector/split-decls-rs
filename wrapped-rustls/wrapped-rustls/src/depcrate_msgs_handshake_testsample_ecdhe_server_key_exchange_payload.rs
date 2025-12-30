// Generated macro for sample_ecdhe_server_key_exchange_payload (function)
macro_rules! Depcrate_msgs_handshake_testsample_ecdhe_server_key_exchange_payload {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_ecdhe_server_key_exchange_payload"}
// Dependencies: {}
fn sample_ecdhe_server_key_exchange_payload () -> ServerKeyExchangePayload { ServerKeyExchangePayload :: Known (ServerKeyExchange { params : ServerKeyExchangeParams :: Ecdh (ServerEcdhParams { curve_params : EcParameters { curve_type : ECCurveType :: NamedCurve , named_group : NamedGroup :: X25519 , } , public : PayloadU8 :: new (vec ! [1 , 2 , 3]) , }) , dss : DigitallySignedStruct :: new (SignatureScheme :: RSA_PSS_SHA256 , vec ! [1 , 2 , 3]) , }) }
};
}
