// Generated macro for impl_264 (impl)
macro_rules! Depcrate_de_implsimpl_264 {
() => {
// Module: crate::de::impls
// Provides: {"impl_264"}
// Dependencies: {}
# [doc = " This impl requires the [`\"rc\"`] Cargo feature of Serde. The resulting"] # [doc = " `Weak<T>` has a reference count of 0 and cannot be upgraded."] # [doc = ""] # [doc = " [`\"rc\"`]: https://serde.rs/feature-flags.html#-features-rc"] # [cfg (all (feature = "rc" , any (feature = "std" , feature = "alloc")))] # [cfg_attr (docsrs , doc (cfg (all (feature = "rc" , any (feature = "std" , feature = "alloc")))))] impl < 'de , T > Deserialize < 'de > for ArcWeak < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { tri ! (Option ::< T >:: deserialize (deserializer)) ; Ok (ArcWeak :: new ()) } }
};
}
