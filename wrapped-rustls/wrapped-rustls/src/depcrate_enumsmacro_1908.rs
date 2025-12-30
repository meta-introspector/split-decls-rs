// Generated macro for macro_1908 (macro)
macro_rules! Depcrate_enumsmacro_1908 {
() => {
// Module: crate::enums
// Provides: {"macro_1908"}
// Dependencies: {}
enum_builder ! { # [doc = " The `ContentType` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u8)] pub enum ContentType { ChangeCipherSpec => 0x14 , Alert => 0x15 , Handshake => 0x16 , ApplicationData => 0x17 , Heartbeat => 0x18 , } }
};
}
