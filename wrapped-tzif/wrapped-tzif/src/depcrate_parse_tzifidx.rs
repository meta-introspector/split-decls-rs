// Generated macro for idx (function)
macro_rules! Depcrate_parse_tzifidx {
() => {
// Module: crate::parse::tzif
// Provides: {"idx"}
// Dependencies: {}
# [doc = " A one-byte unsigned integer specifying a zero-based"] # [doc = " index into the series of time zone designation bytes, thereby"] # [doc = " selecting a particular designation string. Each index MUST be"] # [doc = " in the range [0, charcnt - 1]; it designates the"] # [doc = " NUL-terminated string of bytes starting at position \"idx\" in"] # [doc = " the time zone designations. (This string MAY be empty.) A NUL"] # [doc = " byte MUST exist in the time zone designations at or after"] # [doc = " position \"idx\"."] fn idx < Input > (charcnt : usize) -> impl Parser < Input , Output = usize > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { any () . map (| byte | byte as usize) . then (move | idx | ensure (idx , | & idx | idx < charcnt , "idx should be less than charcnt")) }
};
}
