// Generated macro for impl_417 (impl)
macro_rules! Depcrate_ser_implsimpl_417 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_417"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl Serialize for Path { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match self . to_str () { Some (s) => s . serialize (serializer) , None => Err (Error :: custom ("path contains invalid UTF-8 characters")) , } } }
};
}
