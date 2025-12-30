// Generated macro for internal_components (function)
macro_rules! Depcrate_quirksinternal_components {
() => {
// Module: crate::quirks
// Provides: {"internal_components"}
// Dependencies: {}
# [doc = " Internal component / parsed offsets of the URL."] # [doc = ""] # [doc = " This can be useful for implementing efficient serialization"] # [doc = " for the URL."] # [cfg (feature = "expose_internals")] pub fn internal_components (url : & Url) -> InternalComponents { InternalComponents { scheme_end : url . scheme_end , username_end : url . username_end , host_start : url . host_start , host_end : url . host_end , port : url . port , path_start : url . path_start , query_start : url . query_start , fragment_start : url . fragment_start , } }
};
}
