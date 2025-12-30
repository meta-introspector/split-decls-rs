// Generated macro for impl_280 (impl)
macro_rules! Depcrate_serdeimpl_280 {
() => {
// Module: crate::serde
// Provides: {"impl_280"}
// Dependencies: {}
# [cfg (any (feature = "user" , feature = "system"))] impl Serialize for crate :: Uid { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_newtype_struct ("Uid" , & self . to_string ()) } }
};
}
