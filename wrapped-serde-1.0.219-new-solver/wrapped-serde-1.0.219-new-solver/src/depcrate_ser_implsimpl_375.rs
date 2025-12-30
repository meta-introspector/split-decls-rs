// Generated macro for impl_375 (impl)
macro_rules! Depcrate_ser_implsimpl_375 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_375"}
// Dependencies: {}
impl < Idx > Serialize for RangeInclusive < Idx > where Idx : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { use super :: SerializeStruct ; let mut state = tri ! (serializer . serialize_struct ("RangeInclusive" , 2)) ; tri ! (state . serialize_field ("start" , & self . start ())) ; tri ! (state . serialize_field ("end" , & self . end ())) ; state . end () } }
};
}
