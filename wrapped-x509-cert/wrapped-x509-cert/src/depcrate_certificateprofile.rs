// Generated macro for Profile (trait)
macro_rules! Depcrate_certificateProfile {
() => {
// Module: crate::certificate
// Provides: {"Profile"}
// Dependencies: {}
# [doc = " [`Profile`] allows the consumer of this crate to customize the behavior when parsing"] # [doc = " certificates."] # [doc = " By default, parsing will be made in a rfc5280-compliant manner."] pub trait Profile : PartialEq + Debug + Eq + Ord + Clone + Copy + Default + 'static { # [doc = " Checks to run when parsing serial numbers"] fn check_serial_number (serial : & SerialNumber < Self >) -> der :: Result < () > { if serial . inner . len () > SerialNumber :: < Self > :: MAX_DECODE_LEN { Err (Tag :: Integer . value_error () . into ()) } else { Ok (()) } } # [doc = " Adjustments to the time to run while serializing validity."] # [doc = " See [RFC 5280 Section 4.1.2.5]:"] # [doc = " ```text"] # [doc = " CAs conforming to this profile MUST always encode certificate"] # [doc = " validity dates through the year 2049 as UTCTime; certificate validity"] # [doc = " dates in 2050 or later MUST be encoded as GeneralizedTime."] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 4.1.2.5]: https://www.rfc-editor.org/rfc/rfc5280#section-4.1.2.5"] fn time_encoding (mut time : Time) -> der :: Result < Time > { time . rfc5280_adjust_utc_time () ? ; Ok (time) } }
};
}
