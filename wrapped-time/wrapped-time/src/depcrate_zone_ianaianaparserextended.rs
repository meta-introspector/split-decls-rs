// Generated macro for IanaParserExtended (struct)
macro_rules! Depcrate_zone_ianaIanaParserExtended {
() => {
// Module: crate::zone::iana
// Provides: {"IanaParserExtended"}
// Dependencies: {}
# [doc = " A parser that supplements [`IanaParser`] with about 10kB of additional data to support"] # [doc = " returning canonical and case-normalized IANA time zone IDs."] # [derive (Debug , Clone)] pub struct IanaParserExtended < I > { inner : I , data : DataPayload < TimezoneIdentifiersIanaExtendedV1 > , }
};
}
