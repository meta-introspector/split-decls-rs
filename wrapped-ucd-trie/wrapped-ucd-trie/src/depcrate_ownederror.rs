// Generated macro for Error (enum)
macro_rules! Depcrate_ownedError {
() => {
// Module: crate::owned
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that can occur during construction of a trie."] # [derive (Clone , Debug)] pub enum Error { # [doc = " This error is returned when an invalid codepoint is given to"] # [doc = " `TrieSetOwned::from_codepoints`. An invalid codepoint is a `u32` that"] # [doc = " is greater than `0x10FFFF`."] InvalidCodepoint (u32) , # [doc = " This error is returned when a set of Unicode codepoints could not be"] # [doc = " sufficiently compressed into the trie provided by this crate. There is"] # [doc = " no work-around for this error at this time."] GaveUp , }
};
}
