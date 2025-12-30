// Generated macro for ZeroTrieSimpleAsciiCursor (struct)
macro_rules! Depcrate_cursorZeroTrieSimpleAsciiCursor {
() => {
// Module: crate::cursor
// Provides: {"ZeroTrieSimpleAsciiCursor"}
// Dependencies: {}
# [doc = " A cursor into a [`ZeroTrieSimpleAscii`], useful for stepwise lookup."] # [doc = ""] # [doc = " For examples, see [`ZeroTrieSimpleAscii::cursor()`]."] # [derive (Debug , Clone)] pub struct ZeroTrieSimpleAsciiCursor < 'a > { trie : ZeroTrieSimpleAscii < & 'a [u8] > , }
};
}
