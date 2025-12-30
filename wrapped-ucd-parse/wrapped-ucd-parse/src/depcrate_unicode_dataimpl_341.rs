// Generated macro for impl_341 (impl)
macro_rules! Depcrate_unicode_dataimpl_341 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_341"}
// Dependencies: {}
impl Iterator for CodepointRange { type Item = UnicodeData ; fn next (& mut self) -> Option < UnicodeData > { let cp = match self . range . next () { None => return None , Some (cp) => cp , } ; Some (UnicodeData { codepoint : Codepoint :: from_u32 (cp) . unwrap () , name : "" . to_string () , .. self . start_record . clone () }) } }
};
}
