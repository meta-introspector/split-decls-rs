// Generated macro for impl_973 (impl)
macro_rules! Depcrate_serdeimpl_973 {
() => {
// Module: crate::serde
// Provides: {"impl_973"}
// Dependencies: {}
impl Serialize for UtcDateTime { # [inline] fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { # [cfg (feature = "serde-human-readable")] if serializer . is_human_readable () { let Ok (s) = self . format (& PRIMITIVE_DATE_TIME_FORMAT) else { return Err (S :: Error :: custom ("failed formatting `UtcDateTime`")) ; } ; return serializer . serialize_str (& s) ; } (self . year () , self . ordinal () , self . hour () , self . minute () , self . second () , self . nanosecond () ,) . serialize (serializer) } }
};
}
