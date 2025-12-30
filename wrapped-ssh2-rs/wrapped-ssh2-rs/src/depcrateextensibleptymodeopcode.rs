// Generated macro for ExtensiblePtyModeOpcode (enum)
macro_rules! DepcrateExtensiblePtyModeOpcode {
() => {
// Module: crate
// Provides: {"ExtensiblePtyModeOpcode"}
// Dependencies: {}
# [doc = " An opcode for setting a Pty terminal mode"] # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ExtensiblePtyModeOpcode { # [doc = " Use one of the modes specified by RFC 4250"] Mode (PtyModeOpcode) , # [doc = " Use a mode not reflected by RFC 4250"] Extended (u8) , }
};
}
