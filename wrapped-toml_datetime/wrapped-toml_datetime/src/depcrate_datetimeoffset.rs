// Generated macro for Offset (enum)
macro_rules! Depcrate_datetimeOffset {
() => {
// Module: crate::datetime
// Provides: {"Offset"}
// Dependencies: {}
# [doc = " A parsed TOML time offset"] # [doc = ""] # [derive (PartialEq , Eq , PartialOrd , Ord , Copy , Clone , Debug)] pub enum Offset { # [doc = " > A suffix which, when applied to a time, denotes a UTC offset of 00:00;"] # [doc = " > often spoken \"Zulu\" from the ICAO phonetic alphabet representation of"] # [doc = " > the letter \"Z\". --- [RFC 3339 section 2]"] # [doc = ""] # [doc = " [RFC 3339 section 2]: https://datatracker.ietf.org/doc/html/rfc3339#section-2"] Z , # [doc = " Offset between local time and UTC"] Custom { # [doc = " Minutes: -`1_440..1_440`"] minutes : i16 , } , }
};
}
