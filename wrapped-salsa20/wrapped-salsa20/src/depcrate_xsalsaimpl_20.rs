// Generated macro for impl_20 (impl)
macro_rules! Depcrate_xsalsaimpl_20 {
() => {
// Module: crate::xsalsa
// Provides: {"impl_20"}
// Dependencies: {}
impl < R : Unsigned > StreamCipherCore for XSalsaCore < R > { # [inline (always)] fn remaining_blocks (& self) -> Option < usize > { self . 0 . remaining_blocks () } # [inline (always)] fn process_with_backend (& mut self , f : impl StreamCipherClosure < BlockSize = Self :: BlockSize >) { self . 0 . process_with_backend (f) ; } }
};
}
