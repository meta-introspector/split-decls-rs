// Generated macro for impl_414 (impl)
macro_rules! Depcrate_ser_implsimpl_414 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_414"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] impl Serialize for net :: SocketAddr { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { match * self { net :: SocketAddr :: V4 (ref addr) => addr . serialize (serializer) , net :: SocketAddr :: V6 (ref addr) => addr . serialize (serializer) , } } else { match * self { net :: SocketAddr :: V4 (ref addr) => { serializer . serialize_newtype_variant ("SocketAddr" , 0 , "V4" , addr) } net :: SocketAddr :: V6 (ref addr) => { serializer . serialize_newtype_variant ("SocketAddr" , 1 , "V6" , addr) } } } } }
};
}
