// Generated macro for natural (function)
macro_rules! Depcrate_parse_posixnatural {
() => {
// Module: crate::parse::posix
// Provides: {"natural"}
// Dependencies: {}
# [doc = " Parses a natural number as an i64."] fn natural < Input > () -> impl Parser < Input , Output = i64 > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { digits () . map (| digits | { digits . into_iter () . map (| digit | i64 :: from (digit - b'0')) . rev () . zip (0u32 ..) . map (| (digit , n) | digit * 10i64 . pow (n)) . sum :: < i64 > () }) }
};
}
