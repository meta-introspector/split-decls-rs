// Generated macro for ClientDhParams (struct)
macro_rules! Depcrate_msgs_handshakeClientDhParams {
() => {
// Module: crate::msgs::handshake
// Provides: {"ClientDhParams"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct ClientDhParams { # [doc = " RFC5246: `opaque dh_Yc<1..2^16-1>;`"] pub (crate) public : PayloadU16 < NonEmpty > , }
};
}
