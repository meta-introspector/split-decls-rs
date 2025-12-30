// Generated macro for db (function)
macro_rules! Depcrate_dbdb {
() => {
// Module: crate::db
// Provides: {"db"}
// Dependencies: {}
pub (crate) fn db (args : proc_macro :: TokenStream , input : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { let _nothing = syn :: parse_macro_input ! (args as Nothing) ; let hygiene = Hygiene :: from1 (& input) ; let item = parse_macro_input ! (input as syn :: Item) ; let db_macro = DbMacro { hygiene } ; match db_macro . try_db (item) { Ok (v) => crate :: debug :: dump_tokens ("db" , v) . into () , Err (e) => token_stream_with_error (input , e) , } }
};
}
