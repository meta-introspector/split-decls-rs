// Generated macro for KeyShareEntry (struct)
macro_rules! Depcrate_msgs_handshakeKeyShareEntry {
() => {
// Module: crate::msgs::handshake
// Provides: {"KeyShareEntry"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) struct KeyShareEntry { pub (crate) group : NamedGroup , # [doc = " RFC8446: `opaque key_exchange<1..2^16-1>;`"] pub (crate) payload : PayloadU16 < NonEmpty > , }
};
}
