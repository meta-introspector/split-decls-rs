// Generated macro for step_codes (macro)
macro_rules! Depcrate_binary_formatstep_codes {
() => {
// Module: crate::binary_format
// Provides: {"step_codes"}
// Dependencies: {}
macro_rules ! step_codes { ($ counter : expr , $ name : ident ,) => { const $ name : u64 = $ counter ; } ; ($ counter : expr , $ name : ident , $ ($ names : ident) ,* ,) => { const $ name : u64 = $ counter ; step_codes ! ($ counter + 1 , $ ($ names) ,* ,) ; } ; }
};
}
