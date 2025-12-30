// Generated macro for macro_267 (macro)
macro_rules! Depcrate_de_implsmacro_267 {
() => {
// Module: crate::de::impls
// Provides: {"macro_267"}
// Dependencies: {}
box_forwarded_impl ! { # [doc = " This impl requires the [`\"rc\"`] Cargo feature of Serde."] # [doc = ""] # [doc = " Deserializing a data structure containing `Arc` will not attempt to"] # [doc = " deduplicate `Arc` references to the same data. Every deserialized `Arc`"] # [doc = " will end up with a strong count of 1."] # [doc = ""] # [doc = " [`\"rc\"`]: https://serde.rs/feature-flags.html#-features-rc"] # [cfg (all (feature = "rc" , any (feature = "std" , feature = "alloc")))] # [cfg_attr (docsrs , doc (cfg (all (feature = "rc" , any (feature = "std" , feature = "alloc")))))] Arc }
};
}
