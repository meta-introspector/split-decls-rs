// Generated macro for impl_373 (impl)
macro_rules! Depcrate_ser_implsimpl_373 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_373"}
// Dependencies: {}
impl < Idx > Serialize for Range < Idx > where Idx : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { use super :: SerializeStruct ; let mut state = tri ! (serializer . serialize_struct ("Range" , 2)) ; tri ! (state . serialize_field ("start" , & self . start)) ; tri ! (state . serialize_field ("end" , & self . end)) ; state . end () } }
};
}
