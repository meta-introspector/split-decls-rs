// Generated macro for impl_981 (impl)
macro_rules! Depcrate_serdeimpl_981 {
() => {
// Module: crate::serde
// Provides: {"impl_981"}
// Dependencies: {}
impl Serialize for Weekday { # [inline] fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { # [cfg (feature = "serde-human-readable")] if serializer . is_human_readable () { # [cfg (not (feature = "std"))] use alloc :: string :: ToString ; return self . to_string () . serialize (serializer) ; } self . number_from_monday () . serialize (serializer) } }
};
}
