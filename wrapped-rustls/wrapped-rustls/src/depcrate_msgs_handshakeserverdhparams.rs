// Generated macro for ServerDhParams (struct)
macro_rules! Depcrate_msgs_handshakeServerDhParams {
() => {
// Module: crate::msgs::handshake
// Provides: {"ServerDhParams"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct ServerDhParams { # [doc = " RFC5246: `opaque dh_p<1..2^16-1>;`"] pub (crate) dh_p : PayloadU16 < NonEmpty > , # [doc = " RFC5246: `opaque dh_g<1..2^16-1>;`"] pub (crate) dh_g : PayloadU16 < NonEmpty > , # [doc = " RFC5246: `opaque dh_Ys<1..2^16-1>;`"] pub (crate) dh_ys : PayloadU16 < NonEmpty > , }
};
}
