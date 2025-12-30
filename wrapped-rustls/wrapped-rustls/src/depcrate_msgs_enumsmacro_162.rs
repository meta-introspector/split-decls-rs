// Generated macro for macro_162 (macro)
macro_rules! Depcrate_msgs_enumsmacro_162 {
() => {
// Module: crate::msgs::enums
// Provides: {"macro_162"}
// Dependencies: {}
enum_builder ! { # [doc = " The `ECPointFormat` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u8)] pub enum ECPointFormat { Uncompressed => 0x00 , ANSIX962CompressedPrime => 0x01 , ANSIX962CompressedChar2 => 0x02 , } }
};
}
