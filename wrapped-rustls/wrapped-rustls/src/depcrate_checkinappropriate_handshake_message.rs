// Generated macro for inappropriate_handshake_message (function)
macro_rules! Depcrate_checkinappropriate_handshake_message {
() => {
// Module: crate::check
// Provides: {"inappropriate_handshake_message"}
// Dependencies: {}
pub (crate) fn inappropriate_handshake_message (payload : & MessagePayload < '_ > , content_types : & [ContentType] , handshake_types : & [HandshakeType] ,) -> Error { match payload { MessagePayload :: Handshake { parsed , .. } => { let got_type = parsed . 0 . handshake_type () ; warn ! ("Received a {got_type:?} handshake message while expecting {handshake_types:?}" ,) ; Error :: InappropriateHandshakeMessage { expect_types : handshake_types . to_vec () , got_type , } } payload => inappropriate_message (payload , content_types) , } }
};
}
