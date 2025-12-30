// Generated macro for impl_389 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_389 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_389"}
// Dependencies: {}
impl EchConfigContents { # [doc = " Returns true if there is more than one extension of a given"] # [doc = " type."] pub (crate) fn has_duplicate_extension (& self) -> bool { has_duplicates :: < _ , _ , u16 > (self . extensions . iter () . map (| ext | ext . ext_type ()) ,) } # [doc = " Returns true if there is at least one mandatory unsupported extension."] pub (crate) fn has_unknown_mandatory_extension (& self) -> bool { self . extensions . iter () . any (| ext | { matches ! (ext . ext_type () , ExtensionType :: Unknown (_)) && u16 :: from (ext . ext_type ()) & 0x8000 != 0 }) } }
};
}
