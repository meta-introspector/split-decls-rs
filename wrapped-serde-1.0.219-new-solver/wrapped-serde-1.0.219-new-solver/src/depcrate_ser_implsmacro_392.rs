// Generated macro for macro_392 (macro)
macro_rules! Depcrate_ser_implsmacro_392 {
() => {
// Module: crate::ser::impls
// Provides: {"macro_392"}
// Dependencies: {}
deref_impl ! { # [doc = " This impl requires the [`\"rc\"`] Cargo feature of Serde."] # [doc = ""] # [doc = " Serializing a data structure containing `Rc` will serialize a copy of"] # [doc = " the contents of the `Rc` each time the `Rc` is referenced within the"] # [doc = " data structure. Serialization will not attempt to deduplicate these"] # [doc = " repeated data."] # [doc = ""] # [doc = " [`\"rc\"`]: https://serde.rs/feature-flags.html#-features-rc"] # [cfg (all (feature = "rc" , any (feature = "std" , feature = "alloc")))] # [cfg_attr (docsrs , doc (cfg (all (feature = "rc" , any (feature = "std" , feature = "alloc")))))] < T > Serialize for Rc < T > where T : ? Sized + Serialize }
};
}
