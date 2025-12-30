// Generated macro for impl_408 (impl)
macro_rules! Depcrate_ser_implsimpl_408 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_408"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] impl Serialize for net :: IpAddr { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { match * self { net :: IpAddr :: V4 (ref a) => a . serialize (serializer) , net :: IpAddr :: V6 (ref a) => a . serialize (serializer) , } } else { match * self { net :: IpAddr :: V4 (ref a) => { serializer . serialize_newtype_variant ("IpAddr" , 0 , "V4" , a) } net :: IpAddr :: V6 (ref a) => { serializer . serialize_newtype_variant ("IpAddr" , 1 , "V6" , a) } } } } }
};
}
