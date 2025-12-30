// Generated macro for isutcnt (function)
macro_rules! Depcrate_parse_tzifisutcnt {
() => {
// Module: crate::parse::tzif
// Provides: {"isutcnt"}
// Dependencies: {}
# [doc = " Parse the `TZif` `isutcnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>"] # [doc = " > A four-byte unsigned integer specifying the number of UT/"] # [doc = " > local indicators contained in the data block -- MUST either be"] # [doc = " > zero or equal to \"typecnt\"."] fn isutcnt < Input > () -> impl Parser < Input , Output = usize > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { be_u32 () . map (| u32 | u32 as usize) }
};
}
