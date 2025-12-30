// Generated macro for tinystr (macro)
macro_rules! Depcrate_macrostinystr {
() => {
// Module: crate::macros
// Provides: {"tinystr"}
// Dependencies: {}
# [macro_export] macro_rules ! tinystr { ($ n : literal , $ s : literal) => { { const TINYSTR_MACRO_CONST : $ crate :: TinyAsciiStr <$ n > = { match $ crate :: TinyAsciiStr :: try_from_utf8 ($ s . as_bytes ()) { Ok (s) => s , Err (_) => panic ! (concat ! ("Failed to construct tinystr from " , $ s)) , } } ; TINYSTR_MACRO_CONST } } ; }
};
}
