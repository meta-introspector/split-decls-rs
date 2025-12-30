// Generated macro for impl_240 (impl)
macro_rules! Depcrate_de_implsimpl_240 {
() => {
// Module: crate::de::impls
// Provides: {"impl_240"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] impl < 'de > Deserialize < 'de > for net :: IpAddr { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { deserializer . deserialize_str (FromStrVisitor :: new ("IP address")) } else { use crate :: lib :: net :: IpAddr ; deserialize_enum ! { IpAddr IpAddrKind (V4 ; b"V4" ; 0 , V6 ; b"V6" ; 1) "`V4` or `V6`" , deserializer } } } }
};
}
