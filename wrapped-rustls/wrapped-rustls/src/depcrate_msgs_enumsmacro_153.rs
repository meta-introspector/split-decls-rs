// Generated macro for macro_153 (macro)
macro_rules! Depcrate_msgs_enumsmacro_153 {
() => {
// Module: crate::msgs::enums
// Provides: {"macro_153"}
// Dependencies: {}
enum_builder ! { # [doc = " The `ClientCertificateType` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u8)] pub (crate) enum ClientCertificateType { RSASign => 0x01 , DSSSign => 0x02 , RSAFixedDH => 0x03 , DSSFixedDH => 0x04 , RSAEphemeralDH => 0x05 , DSSEphemeralDH => 0x06 , FortezzaDMS => 0x14 , ECDSASign => 0x40 , RSAFixedECDH => 0x41 , ECDSAFixedECDH => 0x42 , } }
};
}
