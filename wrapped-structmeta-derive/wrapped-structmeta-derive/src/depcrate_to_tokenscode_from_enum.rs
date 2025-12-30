// Generated macro for code_from_enum (function)
macro_rules! Depcrate_to_tokenscode_from_enum {
() => {
// Module: crate::to_tokens
// Provides: {"code_from_enum"}
// Dependencies: {}
fn code_from_enum (data : & DataEnum) -> Result < TokenStream > { let mut arms = Vec :: new () ; for variant in & data . variants { let ident = & variant . ident ; let p = to_pattern (quote ! (Self ::# ident) , & variant . fields) ; let code = code_from_fields (& variant . fields) ? ; arms . push (quote ! { # p => { # code } }) ; } Ok (quote ! { match self { # (# arms) * } }) }
};
}
