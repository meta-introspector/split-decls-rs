// Generated macro for leapcnt (function)
macro_rules! Depcrate_parse_tzifleapcnt {
() => {
// Module: crate::parse::tzif
// Provides: {"leapcnt"}
// Dependencies: {}
# [doc = " Parse the `TZif` `leapcnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>"] # [doc = " > A four-byte unsigned integer specifying the number of"] # [doc = " > leap-second records contained in the data block."] fn leapcnt < Input > () -> impl Parser < Input , Output = usize > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { be_u32 () . map (| u32 | u32 as usize) }
};
}
