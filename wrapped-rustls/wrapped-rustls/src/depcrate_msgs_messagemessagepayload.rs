// Generated macro for MessagePayload (enum)
macro_rules! Depcrate_msgs_messageMessagePayload {
() => {
// Module: crate::msgs::message
// Provides: {"MessagePayload"}
// Dependencies: {}
# [non_exhaustive] # [derive (Debug)] pub enum MessagePayload < 'a > { Alert (AlertMessagePayload) , Handshake { parsed : HandshakeMessagePayload < 'a > , encoded : Payload < 'a > , } , HandshakeFlight (Payload < 'a >) , ChangeCipherSpec (ChangeCipherSpecPayload) , ApplicationData (Payload < 'a >) , }
};
}
