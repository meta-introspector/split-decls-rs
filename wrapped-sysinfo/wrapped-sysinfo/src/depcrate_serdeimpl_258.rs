// Generated macro for impl_258 (impl)
macro_rules! Depcrate_serdeimpl_258 {
() => {
// Module: crate::serde
// Provides: {"impl_258"}
// Dependencies: {}
# [cfg (feature = "system")] impl Serialize for crate :: Pid { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_newtype_struct ("Pid" , & self . to_string ()) } }
};
}
