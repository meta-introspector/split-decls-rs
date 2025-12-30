// Generated macro for impl_376 (impl)
macro_rules! Depcrate_ser_implsimpl_376 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_376"}
// Dependencies: {}
impl < Idx > Serialize for RangeTo < Idx > where Idx : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { use super :: SerializeStruct ; let mut state = tri ! (serializer . serialize_struct ("RangeTo" , 1)) ; tri ! (state . serialize_field ("end" , & self . end)) ; state . end () } }
};
}
