// Generated macro for macro_1909 (macro)
macro_rules! Depcrate_enumsmacro_1909 {
() => {
// Module: crate::enums
// Provides: {"macro_1909"}
// Dependencies: {}
enum_builder ! { # [doc = " The `ProtocolVersion` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u16)] pub enum ProtocolVersion { SSLv2 => 0x0002 , SSLv3 => 0x0300 , TLSv1_0 => 0x0301 , TLSv1_1 => 0x0302 , TLSv1_2 => 0x0303 , TLSv1_3 => 0x0304 , DTLSv1_0 => 0xFEFF , DTLSv1_2 => 0xFEFD , DTLSv1_3 => 0xFEFC , } }
};
}
