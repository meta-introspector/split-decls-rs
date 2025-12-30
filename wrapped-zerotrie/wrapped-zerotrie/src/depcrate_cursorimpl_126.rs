// Generated macro for impl_126 (impl)
macro_rules! Depcrate_cursorimpl_126 {
() => {
// Module: crate::cursor
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'a > ZeroTrieSimpleAscii < & 'a [u8] > { # [doc = " Same as [`ZeroTrieSimpleAscii::cursor()`] but moves self to avoid"] # [doc = " having to doubly anchor the trie to the stack."] # [inline] pub fn into_cursor (self) -> ZeroTrieSimpleAsciiCursor < 'a > { ZeroTrieSimpleAsciiCursor { trie : self } } }
};
}
