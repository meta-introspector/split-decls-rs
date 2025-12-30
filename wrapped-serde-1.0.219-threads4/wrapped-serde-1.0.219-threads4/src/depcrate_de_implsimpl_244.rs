// Generated macro for impl_244 (impl)
macro_rules! Depcrate_de_implsimpl_244 {
() => {
// Module: crate::de::impls
// Provides: {"impl_244"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] impl < 'de > Deserialize < 'de > for net :: SocketAddr { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { deserializer . deserialize_str (FromStrVisitor :: new ("socket address")) } else { use crate :: lib :: net :: SocketAddr ; deserialize_enum ! { SocketAddr SocketAddrKind (V4 ; b"V4" ; 0 , V6 ; b"V6" ; 1) "`V4` or `V6`" , deserializer } } } }
};
}
