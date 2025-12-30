// Generated macro for parse_extensions (function)
macro_rules! Depcrate_attr_parse_metaparse_extensions {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_extensions"}
// Dependencies: {}
pub fn parse_extensions (meta : & CustomMeta , cx : & AttrCtxt ,) -> Result < impl IntoIterator < Item = Extension > , () > { let parser = Punctuated :: < Extension , Token ! [,] > :: parse_terminated ; parse_meta_list_with (meta , cx , parser) }
};
}
