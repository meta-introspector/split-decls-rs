// Generated macro for parse_nested_meta (function)
macro_rules! Depcrate_attr_parse_metaparse_nested_meta {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_nested_meta"}
// Dependencies: {}
pub fn parse_nested_meta (meta : & CustomMeta , cx : & AttrCtxt ,) -> Result < impl IntoIterator < Item = CustomMeta > , () > { let parser = Punctuated :: < CustomMeta , Token ! [,] > :: parse_terminated ; parse_meta_list_with (meta , cx , parser) }
};
}
