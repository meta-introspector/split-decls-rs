// Generated macro for impl_402 (impl)
macro_rules! Depcrate_ser_implsimpl_402 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_402"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < T > Serialize for Mutex < T > where T : ? Sized + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match self . lock () { Ok (locked) => locked . serialize (serializer) , Err (_) => Err (S :: Error :: custom ("lock poison error while serializing")) , } } }
};
}
