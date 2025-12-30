// Generated macro for require_handshake_msg (macro)
macro_rules! Depcrate_checkrequire_handshake_msg {
() => {
// Module: crate::check
// Provides: {"require_handshake_msg"}
// Dependencies: {}
# [doc = " For a Message $m, and a HandshakePayload enum member $payload_type,"] # [doc = " return Ok(payload) if $m is both a handshake message and one that"] # [doc = " has the given $payload_type.  If not, return Err(rustls::Error) quoting"] # [doc = " $handshake_type as the expected handshake type."] macro_rules ! require_handshake_msg (($ m : expr , $ handshake_type : path , $ payload_type : path) => (match &$ m . payload { MessagePayload :: Handshake { parsed : $ crate :: msgs :: handshake :: HandshakeMessagePayload ($ payload_type (hm) ,) , .. } => Ok (hm) , payload => Err ($ crate :: check :: inappropriate_handshake_message (payload , & [$ crate :: enums :: ContentType :: Handshake] , & [$ handshake_type])) })) ;
};
}
