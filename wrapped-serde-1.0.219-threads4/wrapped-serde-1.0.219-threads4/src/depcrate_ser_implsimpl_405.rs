// Generated macro for impl_405 (impl)
macro_rules! Depcrate_ser_implsimpl_405 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_405"}
// Dependencies: {}
impl Serialize for Duration { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { use super :: SerializeStruct ; let mut state = tri ! (serializer . serialize_struct ("Duration" , 2)) ; tri ! (state . serialize_field ("secs" , & self . as_secs ())) ; tri ! (state . serialize_field ("nanos" , & self . subsec_nanos ())) ; state . end () } }
};
}
