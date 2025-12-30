// Generated macro for tag (function)
macro_rules! Depcratetag {
() => {
// Module: crate
// Provides: {"tag"}
// Dependencies: {}
# [doc = " Derive macro generating an implementation of the"] # [doc = " [`Tag`](../tracing_forest/tag/trait.Tag.html) trait."] # [doc = ""] # [doc = " See [`tag` module documentation](../tracing_forest/tag/index.html)"] # [doc = " for details on how to define and use tags."] # [cfg (feature = "derive")] # [proc_macro_derive (Tag , attributes (tag))] pub fn tag (input : TokenStream) -> TokenStream { derive :: tag (input) }
};
}
