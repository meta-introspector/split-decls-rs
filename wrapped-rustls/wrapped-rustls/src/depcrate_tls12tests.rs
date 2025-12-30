// Generated macro for tests (module)
macro_rules! Depcrate_tls12tests {
() => {
// Module: crate::tls12
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: common_state :: { CommonState , Side } ; use crate :: msgs :: handshake :: { ServerEcdhParams , ServerKeyExchangeParams } ; use crate :: { NamedGroup , TEST_PROVIDERS } ; # [test] fn server_ecdhe_remaining_bytes () { for provider in TEST_PROVIDERS { let Some (kx_group) = provider . find_kx_group (NamedGroup :: X25519 , ProtocolVersion :: TLSv1_3) else { continue ; } ; let key = kx_group . start () . unwrap () ; let server_params = ServerEcdhParams :: new (& * key) ; let mut server_buf = Vec :: new () ; server_params . encode (& mut server_buf) ; server_buf . push (34) ; let mut common = CommonState :: new (Side :: Client) ; assert ! (decode_kx_params ::< ServerKeyExchangeParams > (KeyExchangeAlgorithm :: ECDHE , & mut common , & server_buf) . is_err ()) ; } } # [test] fn client_ecdhe_invalid () { let mut common = CommonState :: new (Side :: Server) ; assert ! (decode_kx_params ::< ServerKeyExchangeParams > (KeyExchangeAlgorithm :: ECDHE , & mut common , & [34] ,) . is_err ()) ; } }
};
}
