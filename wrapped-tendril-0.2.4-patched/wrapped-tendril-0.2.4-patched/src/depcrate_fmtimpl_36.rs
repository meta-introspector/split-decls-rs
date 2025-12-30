// Generated macro for impl_36 (impl)
macro_rules! Depcrate_fmtimpl_36 {
() => {
// Module: crate::fmt
// Provides: {"impl_36"}
// Dependencies: {}
unsafe impl < 'a > CharFormat < 'a > for UTF8 { type Iter = str :: CharIndices < 'a > ; # [inline] unsafe fn char_indices (buf : & 'a [u8]) -> str :: CharIndices < 'a > { str :: from_utf8_unchecked (buf) . char_indices () } # [inline] fn encode_char < F > (ch : char , cont : F) -> Result < () , () > where F : FnOnce (& [u8]) { unsafe { let mut utf_8 : [u8 ; 4] = mem :: uninitialized () ; let bytes_written = { let mut buffer = & mut utf_8 [..] ; write ! (buffer , "{}" , ch) . ok () . expect ("Tendril: internal error") ; debug_assert ! (buffer . len () <= 4) ; 4 - buffer . len () } ; cont (unsafe_slice (& utf_8 , 0 , bytes_written)) ; Ok (()) } } }
};
}
