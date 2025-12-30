// Generated macro for isstdcnt (function)
macro_rules! Depcrate_parse_tzifisstdcnt {
() => {
// Module: crate::parse::tzif
// Provides: {"isstdcnt"}
// Dependencies: {}
# [doc = " Parse the `TZif` `isstdcnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>"] # [doc = " > A four-byte unsigned integer specifying the number of"] # [doc = " > standard/wall indicators contained in the data block -- MUST"] # [doc = " > either be zero or equal to \"typecnt\"."] fn isstdcnt < Input > () -> impl Parser < Input , Output = usize > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { be_u32 () . map (| u32 | u32 as usize) }
};
}
