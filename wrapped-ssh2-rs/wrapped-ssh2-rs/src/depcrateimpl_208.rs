// Generated macro for impl_208 (impl)
macro_rules! Depcrateimpl_208 {
() => {
// Module: crate
// Provides: {"impl_208"}
// Dependencies: {}
impl PtyModes { # [doc = " Construct a PtyModes instance so that you can specify values for"] # [doc = " various modes"] pub fn new () -> Self { Self { data : vec ! [] } } # [doc = " Set a mode to an arbitrary u32 value"] pub fn set_u32 < O : Into < ExtensiblePtyModeOpcode > > (& mut self , option : O , value : u32) { let data = [option . into () . as_opcode () , ((value >> 24) & 0xff) as u8 , ((value >> 16) & 0xff) as u8 , ((value >> 8) & 0xff) as u8 , (value & 0xff) as u8 ,] ; self . data . extend_from_slice (& data) ; } # [doc = " Set a mode to a boolean value"] pub fn set_boolean < O : Into < ExtensiblePtyModeOpcode > > (& mut self , option : O , value : bool) { self . set_u32 (option , if value { 1 } else { 0 }) } # [doc = " Set a mode to a character value."] # [doc = " If the character is None it is set to 255 to indicate that it"] # [doc = " is disabled."] # [doc = " While this interface and the protocol accept unicode characters"] # [doc = " of up to 32 bits in width, these options likely only work for"] # [doc = " characters in the 7-bit ascii range."] pub fn set_character < O : Into < ExtensiblePtyModeOpcode > > (& mut self , option : O , c : Option < char >) { self . set_u32 (option , c . map (| c | c as u32) . unwrap_or (255)) } # [doc = " Finish accumulating modes and return the encoded"] # [doc = " byte stream suitable for use in the ssh2 protocol"] pub fn finish (mut self) -> Vec < u8 > { self . data . push (PtyModeOpcode :: TTY_OP_END as u8) ; self . data } }
};
}
