// Generated macro for impl_424 (impl)
macro_rules! Depcrate_msgs_messageimpl_424 {
() => {
// Module: crate::msgs::message
// Provides: {"impl_424"}
// Dependencies: {}
impl Message < '_ > { pub fn build_alert (level : AlertLevel , desc : AlertDescription) -> Self { Self { version : ProtocolVersion :: TLSv1_2 , payload : MessagePayload :: Alert (AlertMessagePayload { level , description : desc , }) , } } pub fn build_key_update_notify () -> Self { Self { version : ProtocolVersion :: TLSv1_3 , payload : MessagePayload :: handshake (HandshakeMessagePayload (HandshakePayload :: KeyUpdate (KeyUpdateRequest :: UpdateNotRequested) ,)) , } } pub fn build_key_update_request () -> Self { Self { version : ProtocolVersion :: TLSv1_3 , payload : MessagePayload :: handshake (HandshakeMessagePayload (HandshakePayload :: KeyUpdate (KeyUpdateRequest :: UpdateRequested) ,)) , } } # [cfg (feature = "std")] pub (crate) fn into_owned (self) -> Message < 'static > { let Self { version , payload } = self ; Message { version , payload : payload . into_owned () , } } # [cfg (test)] pub (crate) fn into_wire_bytes (self) -> Vec < u8 > { PlainMessage :: from (self) . into_unencrypted_opaque () . encode () } pub (crate) fn handshake_type (& self) -> Option < HandshakeType > { match & self . payload { MessagePayload :: Handshake { parsed , .. } => Some (parsed . 0 . handshake_type ()) , _ => None , } } }
};
}
