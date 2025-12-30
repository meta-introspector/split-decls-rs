// Generated macro for impl_284 (impl)
macro_rules! Depcrate_symbolimpl_284 {
() => {
// Module: crate::symbol
// Provides: {"impl_284"}
// Dependencies: {}
impl ByteSymbol { # [doc = " Avoid this except for things like deserialization of previously"] # [doc = " serialized symbols, and testing. Use `intern` instead."] pub const fn new (n : u32) -> Self { ByteSymbol (SymbolIndex :: from_u32 (n)) } # [doc = " Maps a string to its interned representation."] pub fn intern (byte_str : & [u8]) -> Self { with_session_globals (| session_globals | { session_globals . symbol_interner . intern_byte_str (byte_str) }) } # [doc = " Like `Symbol::as_str`."] pub fn as_byte_str (& self) -> & [u8] { with_session_globals (| session_globals | unsafe { std :: mem :: transmute :: < & [u8] , & [u8] > (session_globals . symbol_interner . get_byte_str (* self)) }) } pub fn as_u32 (self) -> u32 { self . 0 . as_u32 () } }
};
}
