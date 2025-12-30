// Generated macro for impl_30 (impl)
macro_rules! Depcrate_fmtimpl_30 {
() => {
// Module: crate::fmt
// Provides: {"impl_30"}
// Dependencies: {}
unsafe impl < 'a > CharFormat < 'a > for ASCII { type Iter = imp :: SingleByteCharIndices < 'a > ; # [inline] unsafe fn char_indices (buf : & 'a [u8]) -> imp :: SingleByteCharIndices < 'a > { imp :: SingleByteCharIndices :: new (buf) } # [inline] fn encode_char < F > (ch : char , cont : F) -> Result < () , () > where F : FnOnce (& [u8]) { let n = ch as u32 ; if n > 0x7F { return Err (()) ; } cont (& [n as u8]) ; Ok (()) } }
};
}
