// Generated macro for charcnt (function)
macro_rules! Depcrate_parse_tzifcharcnt {
() => {
// Module: crate::parse::tzif
// Provides: {"charcnt"}
// Dependencies: {}
# [doc = " Parse the `TZif` `charcnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>"] # [doc = " > A four-byte unsigned integer specifying the total number"] # [doc = " > of bytes used by the set of time zone designations contained in"] # [doc = " > the data block - MUST NOT be zero. The count includes the"] # [doc = " > trailing NUL (0x00) byte at the end of the last time zone"] # [doc = " > designation."] fn charcnt < Input > () -> impl Parser < Input , Output = usize > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { be_u32 () . map (| u32 | u32 as usize) . then (| charcnt | { ensure (charcnt , | & charcnt | charcnt != 0 , "charcnt should never be zero" ,) }) }
};
}
