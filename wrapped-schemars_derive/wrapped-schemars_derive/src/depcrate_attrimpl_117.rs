// Generated macro for impl_117 (impl)
macro_rules! Depcrate_attrimpl_117 {
() => {
// Module: crate::attr
// Provides: {"impl_117"}
// Dependencies: {}
impl Drop for AttrCtxt < '_ > { fn drop (& mut self) { if self . attr_type == "schemars" { for unhandled_meta in self . metas . iter () . filter (| m | ! is_schemars_serde_keyword (m)) { self . error_spanned_by (unhandled_meta . path () , format_args ! ("unknown schemars attribute `{}`" , path_str (unhandled_meta . path ())) ,) ; } } } }
};
}
