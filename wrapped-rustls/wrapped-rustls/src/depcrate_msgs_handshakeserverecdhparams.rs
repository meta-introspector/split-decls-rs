// Generated macro for ServerEcdhParams (struct)
macro_rules! Depcrate_msgs_handshakeServerEcdhParams {
() => {
// Module: crate::msgs::handshake
// Provides: {"ServerEcdhParams"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct ServerEcdhParams { pub (crate) curve_params : EcParameters , # [doc = " RFC4492: `opaque point <1..2^8-1>;`"] pub (crate) public : PayloadU8 < NonEmpty > , }
};
}
