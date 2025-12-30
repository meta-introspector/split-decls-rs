// Generated macro for IanaParserBorrowed (struct)
macro_rules! Depcrate_zone_ianaIanaParserBorrowed {
() => {
// Module: crate::zone::iana
// Provides: {"IanaParserBorrowed"}
// Dependencies: {}
# [doc = " A borrowed wrapper around the time zone ID parser, returned by"] # [doc = " [`IanaParser::as_borrowed()`]. More efficient to query."] # [derive (Debug , Copy , Clone)] pub struct IanaParserBorrowed < 'a > { data : & 'a IanaToBcp47Map < 'a > , checksum : u64 , }
};
}
