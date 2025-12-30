// Generated macro for impl_359 (impl)
macro_rules! Depcrate_ser_implsimpl_359 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_359"}
// Dependencies: {}
impl < T > Serialize for Option < T > where T : Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match * self { Some (ref value) => serializer . serialize_some (value) , None => serializer . serialize_none () , } } }
};
}
