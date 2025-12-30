// Generated macro for timecnt (function)
macro_rules! Depcrate_parse_tziftimecnt {
() => {
// Module: crate::parse::tzif
// Provides: {"timecnt"}
// Dependencies: {}
# [doc = " Parse the `TZif` `timecnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>"] # [doc = " > A four-byte unsigned integer specifying the number of"] # [doc = " > transition times contained in the data block."] fn timecnt < Input > () -> impl Parser < Input , Output = usize > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { be_u32 () . map (| u32 | u32 as usize) }
};
}
