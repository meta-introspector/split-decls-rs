// Generated macro for format_unit_struct (function)
macro_rules! Depcrate_itemsformat_unit_struct {
() => {
// Module: crate::items
// Provides: {"format_unit_struct"}
// Dependencies: {}
fn format_unit_struct (context : & RewriteContext < '_ > , p : & StructParts < '_ > , offset : Indent ,) -> Option < String > { let header_str = format_header (context , p . prefix , p . ident , p . vis , offset) ; let generics_str = if let Some (generics) = p . generics { let hi = context . snippet_provider . span_before_last (p . span , ";") ; format_generics (context , generics , context . config . brace_style () , BracePos :: None , offset , mk_sp (p . ident . span . hi () , hi) , last_line_width (& header_str) ,) ? } else { String :: new () } ; Some (format ! ("{header_str}{generics_str};")) }
};
}
