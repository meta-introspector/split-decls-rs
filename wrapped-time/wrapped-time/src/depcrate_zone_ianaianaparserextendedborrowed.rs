// Generated macro for IanaParserExtendedBorrowed (struct)
macro_rules! Depcrate_zone_ianaIanaParserExtendedBorrowed {
() => {
// Module: crate::zone::iana
// Provides: {"IanaParserExtendedBorrowed"}
// Dependencies: {}
# [doc = " A borrowed wrapper around the time zone ID parser, returned by"] # [doc = " [`IanaParserExtended::as_borrowed()`]. More efficient to query."] # [derive (Debug , Copy , Clone)] pub struct IanaParserExtendedBorrowed < 'a > { inner : IanaParserBorrowed < 'a > , data : & 'a IanaNames < 'a > , }
};
}
