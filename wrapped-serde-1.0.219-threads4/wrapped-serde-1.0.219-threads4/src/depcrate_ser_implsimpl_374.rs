// Generated macro for impl_374 (impl)
macro_rules! Depcrate_ser_implsimpl_374 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_374"}
// Dependencies: {}
impl < Idx > Serialize for RangeFrom < Idx > where Idx : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { use super :: SerializeStruct ; let mut state = tri ! (serializer . serialize_struct ("RangeFrom" , 1)) ; tri ! (state . serialize_field ("start" , & self . start)) ; state . end () } }
};
}
