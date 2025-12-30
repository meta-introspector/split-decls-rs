// Generated macro for TokenSlice (struct)
macro_rules! Depcrate_stream_tokenTokenSlice {
() => {
// Module: crate::stream::token
// Provides: {"TokenSlice"}
// Dependencies: {}
# [doc = " Specialized input for parsing lexed tokens"] # [doc = ""] # [doc = " Helpful impls"] # [doc = " - Any `PartialEq` type (e.g. a `TokenKind` or `&str`) can be used with"] # [doc = "   [`literal`][crate::token::literal]"] # [doc = " - A `PartialEq` for `&str` allows for using `&str` as a parser for tokens"] # [doc = " - [`ContainsToken`][crate::stream::ContainsToken] for `T` to for parsing with token sets"] # [doc = " - [`Location`] for `T` to extract spans from tokens"] # [doc = ""] # [doc = " See also [Lexing and Parsing][crate::_topic::lexing]."] # [derive (Copy , Clone , PartialEq , Eq)] pub struct TokenSlice < 't , T > { initial : & 't [T] , input : & 't [T] , }
};
}
