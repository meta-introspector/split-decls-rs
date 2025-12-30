// Generated macro for impl_984 (impl)
macro_rules! Depcrate_configimpl_984 {
() => {
// Module: crate::config
// Provides: {"impl_984"}
// Dependencies: {}
# [cfg (any (feature = "dist-client" , feature = "dist-server"))] impl Serialize for HTTPUrl { fn serialize < S > (& self , serializer : S) -> StdResult < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (self . 0 . as_str ()) } }
};
}
