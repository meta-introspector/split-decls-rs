// Generated macro for render_to_string (function)
macro_rules! Depcrate_utilsrender_to_string {
() => {
// Module: crate::utils
// Provides: {"render_to_string"}
// Dependencies: {}
pub (crate) fn render_to_string < C , F , E > (context : C , render : F) -> Result < String , Error > where C : FnOnce () -> String , F : FnOnce (& mut Vec < u8 >) -> Result < () , E > , Error : From < E > , { let mut buffer = Vec :: new () ; render (& mut buffer) . map_err (Error :: from) ? ; buffer_to_string (context , buffer) }
};
}
