// Generated macro for macro_207 (macro)
macro_rules! Depcrate_de_implsmacro_207 {
() => {
// Module: crate::de::impls
// Provides: {"macro_207"}
// Dependencies: {}
forwarded_impl ! { # [cfg (any (feature = "std" , all (not (no_core_cstr) , feature = "alloc")))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] () , Box < CStr >, CString :: into_boxed_c_str }
};
}
