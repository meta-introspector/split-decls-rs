// Generated macro for sample_dhe_server_key_exchange_payload (function)
macro_rules! Depcrate_msgs_handshake_testsample_dhe_server_key_exchange_payload {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_dhe_server_key_exchange_payload"}
// Dependencies: {}
fn sample_dhe_server_key_exchange_payload () -> ServerKeyExchangePayload { ServerKeyExchangePayload :: Known (ServerKeyExchange { params : ServerKeyExchangeParams :: Dh (ServerDhParams { dh_p : PayloadU16 :: new (vec ! [1 , 2 , 3]) , dh_g : PayloadU16 :: new (vec ! [2]) , dh_ys : PayloadU16 :: new (vec ! [1 , 2]) , }) , dss : DigitallySignedStruct :: new (SignatureScheme :: RSA_PSS_SHA256 , vec ! [1 , 2 , 3]) , }) }
};
}
