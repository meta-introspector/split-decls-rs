// Generated macro for code_from_fields (function)
macro_rules! Depcrate_to_tokenscode_from_fields {
() => {
// Module: crate::to_tokens
// Provides: {"code_from_fields"}
// Dependencies: {}
fn code_from_fields (fields : & Fields) -> Result < TokenStream > { let mut scopes = vec ! [Scope :: new (None)] ; for (index , field) in fields . iter () . enumerate () { let ident = to_var_ident (Some (index) , & field . ident) ; let mut field_to_tokens = true ; for attr in & field . attrs { if attr . path () . is_ident ("to_tokens") { let attr : ToTokensAttribute = attr . parse_args () ? ; for token in & attr . token { for c in token . value () . chars () { if let Some (delimiter) = delimiter_from_open_char (c) { scopes . push (Scope :: new (Some (Surround { ident : ident . clone () , field , delimiter , }))) ; field_to_tokens = false ; } else if let Some (delimiter) = delimiter_from_close_char (c) { let scope = scopes . pop () . unwrap () ; scopes . last_mut () . unwrap () . ts . extend (scope . into_code (Some (delimiter) , token . span ()) ?) ; } else { bail ! (token . span () , "expected '(', ')', '[', ']', '{{' or '}}', found `{}`." , c) ; } } } } } if field_to_tokens { let code = quote_spanned ! (field . span () => :: structmeta :: helpers :: exports :: quote :: ToTokens :: to_tokens (# ident , tokens) ;) ; scopes . last_mut () . unwrap () . ts . extend (code) ; } } while let Some (scope) = scopes . pop () { if scopes . is_empty () { return Ok (scope . ts) ; } scopes . last_mut () . unwrap () . ts . extend (scope . into_code (None , Span :: call_site ()) ?) ; } unreachable ! () }
};
}
