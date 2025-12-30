// Generated macro for ZeroAsciiIgnoreCaseTrieCursor (struct)
macro_rules! Depcrate_cursorZeroAsciiIgnoreCaseTrieCursor {
() => {
// Module: crate::cursor
// Provides: {"ZeroAsciiIgnoreCaseTrieCursor"}
// Dependencies: {}
# [doc = " A cursor into a [`ZeroAsciiIgnoreCaseTrie`], useful for stepwise lookup."] # [doc = ""] # [doc = " For examples, see [`ZeroAsciiIgnoreCaseTrie::cursor()`]."] # [derive (Debug , Clone)] pub struct ZeroAsciiIgnoreCaseTrieCursor < 'a > { trie : ZeroAsciiIgnoreCaseTrie < & 'a [u8] > , }
};
}
