// Generated macro for impl_983 (impl)
macro_rules! Depcrate_serdeimpl_983 {
() => {
// Module: crate::serde
// Provides: {"impl_983"}
// Dependencies: {}
impl Serialize for Month { # [inline] fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { # [cfg (feature = "serde-human-readable")] if serializer . is_human_readable () { # [cfg (not (feature = "std"))] use alloc :: string :: String ; return self . to_string () . serialize (serializer) ; } u8 :: from (* self) . serialize (serializer) } }
};
}
