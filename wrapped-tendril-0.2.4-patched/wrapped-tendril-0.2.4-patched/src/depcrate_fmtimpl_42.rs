// Generated macro for impl_42 (impl)
macro_rules! Depcrate_fmtimpl_42 {
() => {
// Module: crate::fmt
// Provides: {"impl_42"}
// Dependencies: {}
unsafe impl < 'a > CharFormat < 'a > for Latin1 { type Iter = imp :: SingleByteCharIndices < 'a > ; # [inline] unsafe fn char_indices (buf : & 'a [u8]) -> imp :: SingleByteCharIndices < 'a > { imp :: SingleByteCharIndices :: new (buf) } # [inline] fn encode_char < F > (ch : char , cont : F) -> Result < () , () > where F : FnOnce (& [u8]) { let n = ch as u32 ; if n > 0xFF { return Err (()) ; } cont (& [n as u8]) ; Ok (()) } }
};
}
