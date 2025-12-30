// Generated macro for feature (macro)
macro_rules! Depcrate_macros_cfgfeature {
() => {
// Module: crate::macros::cfg
// Provides: {"feature"}
// Dependencies: {}
# [doc = " Allows specifying arbitrary combinations of features and config flags,"] # [doc = " which are also propagated to `docsrs` config."] # [doc = ""] # [doc = " Each contained item will have the annotations applied"] # [doc = ""] # [doc = " ## Example usage:"] # [doc = " ```no-compile"] # [doc = " feature! {"] # [doc = " #![any("] # [doc = "     feature = \"process\","] # [doc = "     feature = \"sync\","] # [doc = "     feature = \"rt\","] # [doc = "     tokio_unstable"] # [doc = " )]"] # [doc = "     /// docs"] # [doc = "     pub struct MyStruct {};"] # [doc = "     /// docs"] # [doc = "     pub struct AnotherStruct {};"] # [doc = " }"] # [doc = " ```"] # [doc = ""] macro_rules ! feature { (#! [$ meta : meta] $ ($ item : item) *) => { $ (# [cfg ($ meta)] # [cfg_attr (docsrs , doc (cfg ($ meta)))] $ item) * } }
};
}
