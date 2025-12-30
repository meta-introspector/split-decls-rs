// Generated macro for require_handshake_msg_move (macro)
macro_rules! Depcrate_checkrequire_handshake_msg_move {
() => {
// Module: crate::check
// Provides: {"require_handshake_msg_move"}
// Dependencies: {}
# [doc = " Like require_handshake_msg, but moves the payload out of $m."] macro_rules ! require_handshake_msg_move (($ m : expr , $ handshake_type : path , $ payload_type : path) => (match $ m . payload { MessagePayload :: Handshake { parsed : $ crate :: msgs :: handshake :: HandshakeMessagePayload ($ payload_type (hm) ,) , .. } => Ok (hm) , payload => Err ($ crate :: check :: inappropriate_handshake_message (& payload , & [$ crate :: enums :: ContentType :: Handshake] , & [$ handshake_type])) })) ;
};
}
