// Generated macro for optional_str_to_cfref (macro)
macro_rules! Depcrate_authorizationoptional_str_to_cfref {
() => {
// Module: crate::authorization
// Provides: {"optional_str_to_cfref"}
// Dependencies: {}
macro_rules ! optional_str_to_cfref { ($ string : ident) => { { $ string . map (CFString :: new) . map_or (std :: ptr :: null () , | cfs | cfs . as_concrete_TypeRef ()) } } ; }
};
}
