// Generated macro for impl_354 (impl)
macro_rules! Depcrate_ser_implsimpl_354 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_354"}
// Dependencies: {}
impl Serialize for str { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (self) } }
};
}
