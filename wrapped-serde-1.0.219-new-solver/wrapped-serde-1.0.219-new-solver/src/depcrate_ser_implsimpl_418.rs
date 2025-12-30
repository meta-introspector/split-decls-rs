// Generated macro for impl_418 (impl)
macro_rules! Depcrate_ser_implsimpl_418 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_418"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl Serialize for PathBuf { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . as_path () . serialize (serializer) } }
};
}
