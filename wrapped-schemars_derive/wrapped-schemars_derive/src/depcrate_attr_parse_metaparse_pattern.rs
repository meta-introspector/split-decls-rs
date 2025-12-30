// Generated macro for parse_pattern (function)
macro_rules! Depcrate_attr_parse_metaparse_pattern {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_pattern"}
// Dependencies: {}
pub fn parse_pattern (meta : & CustomMeta , cx : & AttrCtxt) -> Result < Expr , () > { parse_meta_list_with (meta , cx , Expr :: parse) }
};
}
