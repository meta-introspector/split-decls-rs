// Generated macro for InlineTable (struct)
macro_rules! Depcrate_inline_tableInlineTable {
() => {
// Module: crate::inline_table
// Provides: {"InlineTable"}
// Dependencies: {}
# [doc = " A TOML [`Value`] that contains a collection of [`Key`]/[`Value`] pairs"] # [derive (Debug , Default , Clone)] pub struct InlineTable { preamble : RawString , pub (crate) implicit : bool , decor : Decor , pub (crate) span : Option < std :: ops :: Range < usize > > , dotted : bool , pub (crate) items : KeyValuePairs , }
};
}
