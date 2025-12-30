// Generated macro for impl_420 (impl)
macro_rules! Depcrate_ser_implsimpl_420 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_420"}
// Dependencies: {}
# [cfg (all (feature = "std" , any (unix , windows)))] # [cfg_attr (docsrs , doc (cfg (all (feature = "std" , any (unix , windows)))))] impl Serialize for OsString { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . as_os_str () . serialize (serializer) } }
};
}
