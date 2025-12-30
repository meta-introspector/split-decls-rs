// Generated macro for impl_357 (impl)
macro_rules! Depcrate_ser_implsimpl_357 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_357"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_cstr)))] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl Serialize for CStr { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self . to_bytes ()) } }
};
}
