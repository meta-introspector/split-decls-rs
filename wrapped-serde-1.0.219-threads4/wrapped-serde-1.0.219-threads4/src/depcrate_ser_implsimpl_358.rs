// Generated macro for impl_358 (impl)
macro_rules! Depcrate_ser_implsimpl_358 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_358"}
// Dependencies: {}
# [cfg (any (feature = "std" , all (not (no_core_cstr) , feature = "alloc")))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl Serialize for CString { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self . to_bytes ()) } }
};
}
