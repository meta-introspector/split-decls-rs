// Generated macro for derive_doc_default (function)
macro_rules! Depcratederive_doc_default {
() => {
// Module: crate
// Provides: {"derive_doc_default"}
// Dependencies: {}
# [doc = " Derives a default instance from the documentation."] fn derive_doc_default (s : synstructure :: Structure) -> TokenStream { let variant = match s . variants () { [variant] => variant , _ => panic ! ("DocDefault requires a struct") , } ; let body = variant . construct (| field , _ | { get_doc_field ("default" , & field . attrs) . map (| expr_str | { expr_str . parse :: < TokenStream > () . expect ("error parsing default expression") }) . unwrap_or_else (| | parse_quote ! (Default :: default ())) }) ; s . gen_impl (quote ! { gen impl Default for @ Self { fn default () -> Self { # body } } }) }
};
}
