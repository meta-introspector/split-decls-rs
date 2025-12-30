// Generated macro for get_meta_items (function)
macro_rules! Depcrate_attrget_meta_items {
() => {
// Module: crate::attr
// Provides: {"get_meta_items"}
// Dependencies: {}
fn get_meta_items (attrs : & [Attribute] , attr_type : & 'static str , cx : & Ctxt) -> Vec < CustomMeta > { let mut result = vec ! [] ; for attr in attrs . iter () . filter (| a | a . path () . is_ident (attr_type)) { match attr . parse_args_with (Punctuated :: < CustomMeta , Token ! [,] > :: parse_terminated) { Ok (list) => result . extend (list) , Err (err) => { if attr_type == "schemars" { cx . syn_error (err) ; } } } } result }
};
}
