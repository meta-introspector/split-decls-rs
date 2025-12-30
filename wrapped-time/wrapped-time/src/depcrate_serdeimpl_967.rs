// Generated macro for impl_967 (impl)
macro_rules! Depcrate_serdeimpl_967 {
() => {
// Module: crate::serde
// Provides: {"impl_967"}
// Dependencies: {}
impl Serialize for OffsetDateTime { # [inline] fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { # [cfg (feature = "serde-human-readable")] if serializer . is_human_readable () { let Ok (s) = self . format (& OFFSET_DATE_TIME_FORMAT) else { return Err (S :: Error :: custom ("failed formatting `OffsetDateTime`")) ; } ; return serializer . serialize_str (& s) ; } (self . year () , self . ordinal () , self . hour () , self . minute () , self . second () , self . nanosecond () , self . offset () . whole_hours () , self . offset () . minutes_past_hour () , self . offset () . seconds_past_minute () ,) . serialize (serializer) } }
};
}
