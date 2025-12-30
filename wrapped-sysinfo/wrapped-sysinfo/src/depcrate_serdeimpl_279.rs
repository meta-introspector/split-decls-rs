// Generated macro for impl_279 (impl)
macro_rules! Depcrate_serdeimpl_279 {
() => {
// Module: crate::serde
// Provides: {"impl_279"}
// Dependencies: {}
# [cfg (any (feature = "user" , feature = "system"))] impl Serialize for crate :: Gid { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_newtype_struct ("Gid" , & self . to_string ()) } }
};
}
