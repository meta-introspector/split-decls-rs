// Generated macro for impl_125 (impl)
macro_rules! Depcrate_cursorimpl_125 {
() => {
// Module: crate::cursor
// Provides: {"impl_125"}
// Dependencies: {}
impl < Store > ZeroAsciiIgnoreCaseTrie < Store > where Store : AsRef < [u8] > + ? Sized , { # [doc = " Gets a cursor into the current trie."] # [doc = ""] # [doc = " Useful to query a trie with data that is not a slice."] # [doc = ""] # [doc = " This is currently supported only on [`ZeroTrieSimpleAscii`]"] # [doc = " and [`ZeroAsciiIgnoreCaseTrie`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Get a value out of a trie by [writing](fmt::Write) it to the cursor:"] # [doc = ""] # [doc = " ```"] # [doc = " use core::fmt::Write;"] # [doc = " use zerotrie::ZeroAsciiIgnoreCaseTrie;"] # [doc = ""] # [doc = " // A trie with two values: \"aBc\" and \"aBcdEf\""] # [doc = " let trie = ZeroAsciiIgnoreCaseTrie::from_bytes(b\"aBc\\x80dEf\\x81\");"] # [doc = ""] # [doc = " // Get out the value for \"abc\" (case-insensitive!)"] # [doc = " let mut cursor = trie.cursor();"] # [doc = " write!(&mut cursor, \"abc\");"] # [doc = " assert_eq!(cursor.take_value(), Some(0));"] # [doc = " ```"] # [doc = ""] # [doc = " For more examples, see [`ZeroTrieSimpleAscii::cursor`]."] # [inline] pub fn cursor (& self) -> ZeroAsciiIgnoreCaseTrieCursor < '_ > { ZeroAsciiIgnoreCaseTrieCursor { trie : self . as_borrowed_slice () , } } }
};
}
