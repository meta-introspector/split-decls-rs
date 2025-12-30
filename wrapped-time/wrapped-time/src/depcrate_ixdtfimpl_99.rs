// Generated macro for impl_99 (impl)
macro_rules! Depcrate_ixdtfimpl_99 {
() => {
// Module: crate::ixdtf
// Provides: {"impl_99"}
// Dependencies: {}
# [allow (deprecated)] impl < A : AsCalendar > ZonedDateTime < A , TimeZoneInfo < models :: Full > > { # [doc = " Create a [`ZonedDateTime`] in any calendar from an RFC 9557 string."] # [deprecated (since = "2.1.0" , note = "use `try_strict_from_str`")] pub fn try_full_from_str (rfc_9557_str : & str , calendar : A , iana_parser : IanaParserBorrowed , offset_calculator : crate :: zone :: VariantOffsetsCalculatorBorrowed ,) -> Result < Self , ParseError > { Self :: try_full_from_utf8 (rfc_9557_str . as_bytes () , calendar , iana_parser , offset_calculator ,) } # [doc = " Create a [`ZonedDateTime`] in any calendar from RFC 9557 UTF-8 bytes."] # [doc = ""] # [doc = " See [`Self::try_full_from_str`]."] # [deprecated (since = "2.1.0" , note = "use `try_strict_from_utf8`")] pub fn try_full_from_utf8 (rfc_9557_str : & [u8] , calendar : A , iana_parser : IanaParserBorrowed , offset_calculator : crate :: zone :: VariantOffsetsCalculatorBorrowed ,) -> Result < Self , ParseError > { let ixdtf_record = IxdtfParser :: from_utf8 (rfc_9557_str) . parse () ? ; let date = Date :: try_from_ixdtf_record (& ixdtf_record , calendar) ? ; let time = Time :: try_from_ixdtf_record (& ixdtf_record) ? ; let zone = Intermediate :: try_from_ixdtf_record (& ixdtf_record) ? . full (iana_parser , offset_calculator) ? ; Ok (ZonedDateTime { date , time , zone }) } }
};
}
