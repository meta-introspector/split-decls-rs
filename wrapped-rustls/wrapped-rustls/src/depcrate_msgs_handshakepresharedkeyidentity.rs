// Generated macro for PresharedKeyIdentity (struct)
macro_rules! Depcrate_msgs_handshakePresharedKeyIdentity {
() => {
// Module: crate::msgs::handshake
// Provides: {"PresharedKeyIdentity"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) struct PresharedKeyIdentity { # [doc = " RFC8446: `opaque identity<1..2^16-1>;`"] pub (crate) identity : PayloadU16 < NonEmpty > , pub (crate) obfuscated_ticket_age : u32 , }
};
}
