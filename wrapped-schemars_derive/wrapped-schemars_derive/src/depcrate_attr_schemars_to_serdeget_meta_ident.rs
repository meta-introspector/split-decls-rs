// Generated macro for get_meta_ident (function)
macro_rules! Depcrate_attr_schemars_to_serdeget_meta_ident {
() => {
// Module: crate::attr::schemars_to_serde
// Provides: {"get_meta_ident"}
// Dependencies: {}
fn get_meta_ident (meta : & CustomMeta) -> Option < String > { meta . path () . get_ident () . map (std :: string :: ToString :: to_string) }
};
}
