// Generated macro for utoff (function)
macro_rules! Depcrate_parse_tzifutoff {
() => {
// Module: crate::parse::tzif
// Provides: {"utoff"}
// Dependencies: {}
# [doc = " A four-byte signed integer specifying the number of"] # [doc = " seconds to be added to UT in order to determine local time."] # [doc = " The value MUST NOT be -2**31 and SHOULD be in the range"] # [doc = " [-89999, 93599] (i.e., its value SHOULD be more than -25 hours"] # [doc = " and less than 26 hours). Avoiding -2**31 allows 32-bit clients"] # [doc = " to negate the value without overflow. Restricting it to"] # [doc = " [-89999, 93599] allows easy support by implementations that"] # [doc = " already support the POSIX-required range [-24:59:59, 25:59:59]."] fn utoff < Input > () -> impl Parser < Input , Output = Seconds > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { be_i32 () . then (| utoff | { ensure (utoff , | & utoff | utoff != (- 2i32) . pow (31) , "utoff should never be equal to -2.pow(31)" ,) }) . map (| utoff | Seconds (i64 :: from (utoff))) }
};
}
