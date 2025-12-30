// Generated macro for impl_976 (impl)
macro_rules! Depcrate_serdeimpl_976 {
() => {
// Module: crate::serde
// Provides: {"impl_976"}
// Dependencies: {}
impl Serialize for Time { # [inline] fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { # [cfg (feature = "serde-human-readable")] if serializer . is_human_readable () { let Ok (s) = self . format (& TIME_FORMAT) else { return Err (S :: Error :: custom ("failed formatting `Time`")) ; } ; return serializer . serialize_str (& s) ; } (self . hour () , self . minute () , self . second () , self . nanosecond ()) . serialize (serializer) } }
};
}
