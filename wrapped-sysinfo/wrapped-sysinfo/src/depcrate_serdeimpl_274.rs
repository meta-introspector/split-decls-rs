// Generated macro for impl_274 (impl)
macro_rules! Depcrate_serdeimpl_274 {
() => {
// Module: crate::serde
// Provides: {"impl_274"}
// Dependencies: {}
# [cfg (feature = "network")] impl Serialize for crate :: MacAddr { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_newtype_struct ("MacAddr" , & self . 0) } }
};
}
