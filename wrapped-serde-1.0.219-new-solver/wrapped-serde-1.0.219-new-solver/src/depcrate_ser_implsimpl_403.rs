// Generated macro for impl_403 (impl)
macro_rules! Depcrate_ser_implsimpl_403 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_403"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < T > Serialize for RwLock < T > where T : ? Sized + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match self . read () { Ok (locked) => locked . serialize (serializer) , Err (_) => Err (S :: Error :: custom ("lock poison error while serializing")) , } } }
};
}
