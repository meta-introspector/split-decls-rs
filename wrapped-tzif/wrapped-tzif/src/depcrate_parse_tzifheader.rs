// Generated macro for header (function)
macro_rules! Depcrate_parse_tzifheader {
() => {
// Module: crate::parse::tzif
// Provides: {"header"}
// Dependencies: {}
# [doc = " Parse a `TZif` file header specified by <https://datatracker.ietf.org/doc/html/rfc8536>"] # [doc = " > A `TZif` header is structured as follows (the lengths of multi-byte"] # [doc = " > fields are shown in parentheses):"] # [doc = " > ```text"] # [doc = " > +---------------+---+"] # [doc = " > |  magic    (4) |ver|"] # [doc = " > +---------------+---+---------------------------------------+"] # [doc = " > |           [unused - reserved for future use] (15)         |"] # [doc = " > +---------------+---------------+---------------+-----------+"] # [doc = " > |  isutcnt  (4) |  isstdcnt (4) |  leapcnt  (4) |"] # [doc = " > +---------------+---------------+---------------+"] # [doc = " > |  timecnt  (4) |  typecnt  (4) |  charcnt  (4) |"] # [doc = " > +---------------+---------------+---------------+"] # [doc = " > ```"] fn header < Input > () -> impl Parser < Input , Output = TzifHeader > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { magic_sequence () . with ((version () , skip_count (15 , any ()) . with (isutcnt ()) , isstdcnt () , leapcnt () , timecnt () ,)) . then (| (version , isutcnt , isstdcnt , leapcnt , timecnt) | { combine :: struct_parser ! { TzifHeader { version : value (version) , isutcnt : value (isutcnt) , isstdcnt : value (isstdcnt) , leapcnt : value (leapcnt) , timecnt : value (timecnt) , typecnt : typecnt (isutcnt , isstdcnt) , charcnt : charcnt () , } } }) }
};
}
