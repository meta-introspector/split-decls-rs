// Generated macro for parse_lit_str (function)
macro_rules! Depcrate_attr_parse_metaparse_lit_str {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_lit_str"}
// Dependencies: {}
fn parse_lit_str < T : Parse > (lit_str : & LitStr , cx : & AttrCtxt) -> Result < T , () > { lit_str . parse () . map_err (| _ | { cx . error_spanned_by (lit_str , format_args ! ("failed to parse \"{}\" as a {}" , lit_str . value () , std :: any :: type_name ::< T > () . rsplit ("::") . next () . unwrap_or_default () . to_ascii_lowercase () ,) ,) ; }) }
};
}
