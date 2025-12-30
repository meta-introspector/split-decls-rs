// Generated macro for PskKeyExchangeModes (struct)
macro_rules! Depcrate_msgs_handshakePskKeyExchangeModes {
() => {
// Module: crate::msgs::handshake
// Provides: {"PskKeyExchangeModes"}
// Dependencies: {}
# [doc = " RFC8446: `PskKeyExchangeMode ke_modes<1..255>;`"] # [derive (Clone , Copy , Debug , Default)] pub (crate) struct PskKeyExchangeModes { pub (crate) psk_dhe : bool , pub (crate) psk : bool , }
};
}
