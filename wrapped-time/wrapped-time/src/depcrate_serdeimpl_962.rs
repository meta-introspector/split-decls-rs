// Generated macro for impl_962 (impl)
macro_rules! Depcrate_serdeimpl_962 {
() => {
// Module: crate::serde
// Provides: {"impl_962"}
// Dependencies: {}
impl Serialize for Date { # [inline] fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { # [cfg (feature = "serde-human-readable")] if serializer . is_human_readable () { let Ok (s) = self . format (& DATE_FORMAT) else { return Err (S :: Error :: custom ("failed formatting `Date`")) ; } ; return serializer . serialize_str (& s) ; } (self . year () , self . ordinal ()) . serialize (serializer) } }
};
}
