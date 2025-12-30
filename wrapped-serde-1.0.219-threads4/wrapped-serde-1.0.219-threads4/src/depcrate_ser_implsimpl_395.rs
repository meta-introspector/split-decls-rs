// Generated macro for impl_395 (impl)
macro_rules! Depcrate_ser_implsimpl_395 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_395"}
// Dependencies: {}
# [doc = " This impl requires the [`\"rc\"`] Cargo feature of Serde."] # [doc = ""] # [doc = " [`\"rc\"`]: https://serde.rs/feature-flags.html#-features-rc"] # [cfg (all (feature = "rc" , any (feature = "std" , feature = "alloc")))] # [cfg_attr (docsrs , doc (cfg (all (feature = "rc" , any (feature = "std" , feature = "alloc")))))] impl < T > Serialize for RcWeak < T > where T : ? Sized + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . upgrade () . serialize (serializer) } }
};
}
