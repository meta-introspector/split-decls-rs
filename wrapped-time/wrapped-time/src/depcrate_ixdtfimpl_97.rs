// Generated macro for impl_97 (impl)
macro_rules! Depcrate_ixdtfimpl_97 {
() => {
// Module: crate::ixdtf
// Provides: {"impl_97"}
// Dependencies: {}
impl < A : AsCalendar > ZonedDateTime < A , UtcOffset > { # [doc = " Create a [`ZonedDateTime`] in any calendar from an RFC 9557 string."] # [doc = ""] # [doc = " Returns an error if the string has a calendar annotation that does not"] # [doc = " match the calendar argument, unless the argument is [`Iso`]."] # [doc = ""] # [doc = " This function is \"strict\": the string should have only an offset and no named time zone."] pub fn try_offset_only_from_str (rfc_9557_str : & str , calendar : A) -> Result < Self , ParseError > { Self :: try_offset_only_from_utf8 (rfc_9557_str . as_bytes () , calendar) } # [doc = " Create a [`ZonedDateTime`] in any calendar from RFC 9557 syntax UTF-8 bytes."] # [doc = ""] # [doc = " See [`Self:try_offset_only_from_str`](Self::try_offset_only_from_str)."] pub fn try_offset_only_from_utf8 (rfc_9557_str : & [u8] , calendar : A) -> Result < Self , ParseError > { let ixdtf_record = IxdtfParser :: from_utf8 (rfc_9557_str) . parse () ? ; let date = Date :: try_from_ixdtf_record (& ixdtf_record , calendar) ? ; let time = Time :: try_from_ixdtf_record (& ixdtf_record) ? ; let zone = Intermediate :: try_from_ixdtf_record (& ixdtf_record) ? . offset_only () ? ; Ok (ZonedDateTime { date , time , zone }) } }
};
}
