// Generated macro for impl_979 (impl)
macro_rules! Depcrate_serdeimpl_979 {
() => {
// Module: crate::serde
// Provides: {"impl_979"}
// Dependencies: {}
impl Serialize for UtcOffset { # [inline] fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { # [cfg (feature = "serde-human-readable")] if serializer . is_human_readable () { let Ok (s) = self . format (& UTC_OFFSET_FORMAT) else { return Err (S :: Error :: custom ("failed formatting `UtcOffset`")) ; } ; return serializer . serialize_str (& s) ; } (self . whole_hours () , self . minutes_past_hour () , self . seconds_past_minute () ,) . serialize (serializer) } }
};
}
