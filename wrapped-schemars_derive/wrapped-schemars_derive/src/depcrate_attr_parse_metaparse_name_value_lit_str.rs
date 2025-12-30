// Generated macro for parse_name_value_lit_str (function)
macro_rules! Depcrate_attr_parse_metaparse_name_value_lit_str {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_name_value_lit_str"}
// Dependencies: {}
pub fn parse_name_value_lit_str < T : Parse > (meta : CustomMeta , cx : & AttrCtxt) -> Result < T , () > { let lit_str = require_name_value_lit_str (meta , cx) ? ; parse_lit_str (& lit_str , cx) }
};
}
