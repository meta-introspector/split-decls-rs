// Generated macro for code_from_struct (function)
macro_rules! Depcrate_to_tokenscode_from_struct {
() => {
// Module: crate::to_tokens
// Provides: {"code_from_struct"}
// Dependencies: {}
fn code_from_struct (data : & DataStruct) -> Result < TokenStream > { let p = to_pattern (quote ! (Self) , & data . fields) ; let ts = code_from_fields (& data . fields) ? ; let ts = quote ! { let # p = self ; # ts } ; Ok (ts) }
};
}
