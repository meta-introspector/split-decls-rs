// Generated macro for impl_361 (impl)
macro_rules! Depcrate_ser_implsimpl_361 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_361"}
// Dependencies: {}
impl < T > Serialize for [T ; 0] { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { tri ! (serializer . serialize_tuple (0)) . end () } }
};
}
