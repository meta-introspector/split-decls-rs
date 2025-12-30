// Generated macro for macro_394 (macro)
macro_rules! Depcrate_ser_implsmacro_394 {
() => {
// Module: crate::ser::impls
// Provides: {"macro_394"}
// Dependencies: {}
deref_impl ! { # [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] <'a , T > Serialize for Cow <'a , T > where T : ? Sized + Serialize + ToOwned }
};
}
