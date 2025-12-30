// Generated macro for impl_127 (impl)
macro_rules! Depcrate_cursorimpl_127 {
() => {
// Module: crate::cursor
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'a > ZeroAsciiIgnoreCaseTrie < & 'a [u8] > { # [doc = " Same as [`ZeroAsciiIgnoreCaseTrie::cursor()`] but moves self to avoid"] # [doc = " having to doubly anchor the trie to the stack."] # [inline] pub fn into_cursor (self) -> ZeroAsciiIgnoreCaseTrieCursor < 'a > { ZeroAsciiIgnoreCaseTrieCursor { trie : self } } }
};
}
