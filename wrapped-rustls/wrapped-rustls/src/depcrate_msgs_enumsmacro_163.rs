// Generated macro for macro_163 (macro)
macro_rules! Depcrate_msgs_enumsmacro_163 {
() => {
// Module: crate::msgs::enums
// Provides: {"macro_163"}
// Dependencies: {}
enum_builder ! { # [doc = " The `HeartbeatMode` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u8)] pub (crate) enum HeartbeatMode { PeerAllowedToSend => 0x01 , PeerNotAllowedToSend => 0x02 , } }
};
}
