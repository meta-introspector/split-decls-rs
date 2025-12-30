// Generated macro for parse_macro_input (macro)
macro_rules! Depcrateparse_macro_input {
() => {
// Module: crate
// Provides: {"parse_macro_input"}
// Dependencies: {}
# [doc = " Similar to `syn::parse_macro_input`, however, when a parse error is encountered, it will return"] # [doc = " the input token stream in addition to the error. This will make it so that rust-analyzer can work"] # [doc = " with incomplete code."] macro_rules ! parse_macro_input { ($ tokenstream : ident as $ ty : ty) => { match syn :: parse ::<$ ty > ($ tokenstream . clone ()) { Ok (data) => data , Err (err) => { return $ crate :: token_stream_with_error ($ tokenstream , err) ; } } } ; }
};
}
