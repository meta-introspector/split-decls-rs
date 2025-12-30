// Generated macro for impl_275 (impl)
macro_rules! Depcrate_serdeimpl_275 {
() => {
// Module: crate::serde
// Provides: {"impl_275"}
// Dependencies: {}
# [cfg (feature = "network")] impl Serialize for crate :: IpNetwork { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("IpNetwork" , 2) ? ; state . serialize_field ("addr" , & self . addr) ? ; state . serialize_field ("prefix" , & self . prefix) ? ; state . end () } }
};
}
