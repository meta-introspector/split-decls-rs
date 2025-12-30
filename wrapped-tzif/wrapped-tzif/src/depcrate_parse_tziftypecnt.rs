// Generated macro for typecnt (function)
macro_rules! Depcrate_parse_tziftypecnt {
() => {
// Module: crate::parse::tzif
// Provides: {"typecnt"}
// Dependencies: {}
# [doc = " Parse the `TZif` `typecnt` value specified by <https://datatracker.ietf.org/doc/html/rfc8536>"] # [doc = " > A four-byte unsigned integer specifying the number of"] # [doc = " > local time type records contained in the data block -- MUST NOT be"] # [doc = " > zero. (Although local time type records convey no useful"] # [doc = " > information in files that have nonempty TZ strings but no"] # [doc = " > transitions, at least one such record is nevertheless required"] # [doc = " > because many `TZif` readers reject files that have zero time types.)"] # [doc = ""] # [doc = " Takes `isutcnt` and `isstdcnt` as arguments. If either of these values are"] # [doc = " non-zero, then they must be equal to the parsed `typecnt`."] fn typecnt < Input > (isutcnt : usize , isstdcnt : usize) -> impl Parser < Input , Output = usize > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { be_u32 () . map (| u32 | u32 as usize) . then (| typecnt | { ensure (typecnt , | & typecnt | typecnt != 0 , "typecnt should never be equal to zero" ,) }) . then (move | typecnt | { ensure (typecnt , | & typecnt | isutcnt == 0 || isutcnt == typecnt , "if isutcnt is non-zero it should be equal to typecnt" ,) }) . then (move | typecnt | { ensure (typecnt , | & typecnt | isstdcnt == 0 || isstdcnt == typecnt , "if isstdcnt is non-zero it should be equal to typecnt" ,) }) }
};
}
