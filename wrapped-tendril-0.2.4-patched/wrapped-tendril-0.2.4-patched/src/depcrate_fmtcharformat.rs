// Generated macro for CharFormat (trait)
macro_rules! Depcrate_fmtCharFormat {
() => {
// Module: crate::fmt
// Provides: {"CharFormat"}
// Dependencies: {}
# [doc = " Indicates a format which contains characters from Unicode"] # [doc = " (all of it, or some proper subset)."] pub unsafe trait CharFormat < 'a > : Format { # [doc = " Iterator for characters and their byte indices."] type Iter : Iterator < Item = (usize , char) > ; # [doc = " Iterate over the characters of the string and their byte"] # [doc = " indices."] # [doc = ""] # [doc = " You may assume the buffer is *already validated* for `Format`."] unsafe fn char_indices (buf : & 'a [u8]) -> Self :: Iter ; # [doc = " Encode the character as bytes and pass them to a continuation."] # [doc = ""] # [doc = " Returns `Err(())` iff the character cannot be represented."] fn encode_char < F > (ch : char , cont : F) -> Result < () , () > where F : FnOnce (& [u8]) ; }
};
}
