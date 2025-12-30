// Generated macro for impl_355 (impl)
macro_rules! Depcrate_ser_implsimpl_355 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_355"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl Serialize for String { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (self) } }
};
}
