// Generated macro for sample_server_hello_payload (function)
macro_rules! Depcrate_msgs_handshake_testsample_server_hello_payload {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_server_hello_payload"}
// Dependencies: {}
fn sample_server_hello_payload () -> ServerHelloPayload { ServerHelloPayload { legacy_version : ProtocolVersion :: TLSv1_2 , random : Random :: from ([0 ; 32]) , session_id : SessionId :: empty () , cipher_suite : CipherSuite :: TLS_PSK_WITH_AES_128_CCM , compression_method : Compression :: Null , extensions : Box :: new (ServerExtensions { ec_point_formats : Some (SupportedEcPointFormats :: default ()) , server_name_ack : Some (()) , session_ticket_ack : Some (()) , renegotiation_info : Some (PayloadU8 :: new (vec ! [0])) , selected_protocol : Some (SingleProtocolName :: new (ProtocolName :: from (vec ! [0]))) , key_share : Some (KeyShareEntry :: new (NamedGroup :: X25519 , & [1 , 2 , 3] [..])) , preshared_key : Some (3) , early_data_ack : Some (()) , encrypted_client_hello_ack : Some (ServerEncryptedClientHello { retry_configs : vec ! [] , }) , extended_master_secret_ack : Some (()) , certificate_status_request_ack : Some (()) , selected_version : Some (ProtocolVersion :: TLSv1_2) , transport_parameters : Some (Payload :: new (vec ! [1 , 2 , 3])) , client_certificate_type : Some (CertificateType :: RawPublicKey) , server_certificate_type : Some (CertificateType :: RawPublicKey) , unknown_extensions : Default :: default () , }) , } }
};
}
